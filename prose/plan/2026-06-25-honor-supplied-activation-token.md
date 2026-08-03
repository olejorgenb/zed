# Plan: honor a supplied xdg-activation token from the CLI (focus-on-open, Wayland)

Status: **proposed** (Zed soft-fork feature). Pinned against Zed `17c0ebb0f7`.

## Context for a fresh session

- This is a prospective **soft-fork** of Zed, kept re-mergeable with upstream, so
  the guiding constraint is **prefer additive changes** (new fields, new trait
  methods with default impls, new match arms) over edits to existing functions.
  The only unavoidable edit to an existing function lives in the fork-owned Linux
  backend (`gpui_linux`).
- **Line numbers are anchors pinned to `17c0ebb0f7` and will drift.** Treat every
  `file:line` as "find this symbol near here" — grep by the named symbol
  (`CliRequest::Open`, `handle_cli_connection`, `activate_window`,
  `PlatformWindow::activate`, `set_pending_activation`, `PendingActivation`).
- Repo conventions are in `CLAUDE.md` at the repo root. Build/lint: use
  **`./script/clippy`** (not `cargo clippy`). The crates touched are `cli`,
  `zed`, `gpui`, and `gpui_linux`.
- The Linux platform backend is a **separate crate**, `gpui_linux`
  (`crates/gpui_linux/src/linux/...`), not `crates/gpui/src/platform/linux`.
- **Companion piece (different repo):** the niri-side request
  `prose/request/2026-06-25-xdg-activation-window-rule.md` in the piri fork (a
  `window-rule` property — e.g. `allow-self-activation true` — that makes niri
  honor a matched window's stale-serial self-activation). The two are
  complementary; see `## Scope` for exactly what each fixes.

## Goal

When the running Zed instance is asked to open paths from a context that holds a
valid `xdg-activation` token (e.g. launched via `niri msg action spawn`, a
`.desktop` launcher, or any compositor/launcher that sets `XDG_ACTIVATION_TOKEN`),
**redeem that supplied token to take focus**, instead of self-minting a token
with a stale serial that the compositor rejects.

Today the Wayland backend always self-mints (`gpui_linux/.../wayland/window.rs:1272`):

```rust
fn activate(&self) {
    // Try to request an activation token. Even though the activation is likely going to be rejected,
    // KWin and Mutter can use the app_id to visually indicate we're requesting attention.
    ...
    let token = activation.get_activation_token(&state.globals.qh, ());
    let serial = state.client.get_serial(SerialKind::MousePress); // stale: this window is unfocused
    ...
}
```

The instance is unfocused, so its `MousePress` serial is stale and focus-stealing
prevention rejects the activation. A token minted by the *focused* launching
context carries a valid serial; we just need to plumb it through and redeem it.

## The constraint that shapes the design

**The CLI's `env` HashMap is `None` exactly when we need the token.** On Linux,
`crates/cli/src/main.rs:595` sets `env = None` whenever stdout is **not** a
terminal:

```rust
if !std::io::stdout().is_terminal() {
    None                                   // <-- niri `spawn` nulls stdio → this branch
} else {
    Some(std::env::vars().collect())
}
```

niri's `Action::Spawn` redirects child stdio to `/dev/null`
(niri `src/utils/spawning.rs`), so a `zed` launched via `niri msg action spawn`
hits the `None` branch and the forwarded env — including any
`XDG_ACTIVATION_TOKEN` niri just set — is dropped. Desktop-entry launches are
non-tty too.

**Therefore the token must NOT ride in the `env` map.** Add a dedicated
`activation_token: Option<String>` field to the CLI request, populated directly
from the env var on the CLI side, independent of the `is_terminal()` gate.

## Design

Three small, mostly-additive pieces: a CLI request field, a gpui "next
activation token" side-channel, and the Wayland `activate()` consume-branch.

### 1. CLI: capture and forward the token (`crates/cli`)

- `crates/cli/src/cli.rs:58` — add to `CliRequest::Open`:
  ```rust
  #[serde(default)]
  activation_token: Option<String>,
  ```
  (`#[serde(default)]` keeps the wire format back-compatible with older daemons.)
- `crates/cli/src/main.rs:713` — populate it when building the request, read
  **directly** from the environment, not from the `env` map:
  ```rust
  activation_token: std::env::var("XDG_ACTIVATION_TOKEN").ok(),
  ```
  Per freedesktop convention, also `std::env::remove_var("XDG_ACTIVATION_TOKEN")`
  after reading so it isn't inherited by anything the CLI subsequently spawns.
  (Optional fallback: `DESKTOP_STARTUP_ID` for X11 startup-notification; out of
  scope here — see `## Scope`.)

### 2. gpui: a "next activation token" side-channel

The activation that actually focuses on Wayland is the **window-level**
`Window::activate_window()` (`crates/gpui/src/window.rs:5161` →
`PlatformWindow::activate()`), reached from `workspace::open_paths`
(`crates/workspace/src/workspace.rs:10118` existing window, `:10174` new window).
App-level `cx.activate(true)` (`open_listener.rs:636`) is incidental on Wayland.

To avoid threading a token argument through `OpenOptions` → `open_paths` → every
`activate_window` call site, latch it as a one-shot platform value consumed by
the next `activate()`:

- `crates/gpui/src/platform.rs:122` (`trait Platform`) — add with a **default
  no-op** so macOS/Windows/X11/test need no change:
  ```rust
  fn set_next_activation_token(&self, _token: Option<String>) {}
  ```
- `crates/gpui/src/app.rs` (near `App::activate`, `:1189`):
  ```rust
  pub fn set_next_activation_token(&self, token: Option<String>) {
      self.platform.set_next_activation_token(token);
  }
  ```

### 3. gpui_linux Wayland: store + consume the token

- Add a one-shot field to the shared client state, e.g.
  `provided_activation_token: RefCell<Option<String>>` (alongside the existing
  `pending_activation` machinery in `crates/gpui_linux/src/linux/wayland/client.rs`).
- Implement `Platform::set_next_activation_token` on the Wayland platform to
  store into it.
- Edit `PlatformWindow::activate()` (`.../wayland/window.rs:1272`) to consume it
  before self-minting:
  ```rust
  fn activate(&self) {
      let state = self.borrow();
      if let Some(token) = state.client.take_provided_activation_token() {
          if let Some(activation) = &state.globals.activation {
              // Redeem the externally-supplied (valid) token directly — no
              // get_activation_token round-trip needed.
              activation.activate(&token, &state.surface);
              return;
          }
      }
      // ...existing self-mint fallback unchanged...
  }
  ```
  This is the single edit to an existing function, and it's in fork-owned code.

### 4. Daemon: set the token before opening (`crates/zed`)

- `crates/zed/src/zed/open_listener.rs` `handle_cli_connection` (`:562`,
  activation at `:636`) — destructure `activation_token` from the request and, if
  `Some`, call `cx.set_next_activation_token(Some(token))` right before the open
  proceeds. The subsequent `window.activate_window()` inside `open_paths`
  consumes it (first-activate-wins).

## File-by-file change list

| File | Change |
|---|---|
| `crates/cli/src/cli.rs` | `activation_token: Option<String>` on `CliRequest::Open` (`#[serde(default)]`) |
| `crates/cli/src/main.rs` | read `XDG_ACTIVATION_TOKEN` directly into the field; `remove_var` after |
| `crates/gpui/src/platform.rs` | `Platform::set_next_activation_token` (default no-op) |
| `crates/gpui/src/app.rs` | `App::set_next_activation_token` forwarding to platform |
| `crates/gpui_linux/.../wayland/client.rs` | one-shot `provided_activation_token` field + setter/taker; impl platform method |
| `crates/gpui_linux/.../wayland/window.rs` | consume-token branch in `activate()` (the one existing-fn edit) |
| `crates/zed/src/zed/open_listener.rs` | set the token in `handle_cli_connection` before open |

## Soft-fork posture

- Everything except the Wayland `activate()` branch is additive (new field, new
  trait method with default impl, new struct field). Non-Linux platforms compile
  unchanged via the default no-op.
- The CLI field is `#[serde(default)]`, so a forked CLI talks to an upstream
  daemon (token ignored) and vice-versa without breakage.
- Upstreamability: the whole change is a clean correctness fix ("honor a supplied
  activation token instead of self-minting a doomed one") and is worth offering
  upstream to shrink the fork. The side-channel API is the only debatable bit; an
  explicit `Window::activate_window_with_token` (see alternative) may be more
  palatable upstream.

### Alternative: explicit API instead of a side-channel

Add `PlatformWindow::activate_with_token(&self, token: &str)` (default impl →
`self.activate()`) and `Window::activate_window_with_token`, then thread the
token from the request through `OpenOptions` into the `open_paths` activate call
sites (`workspace.rs:10118`/`:10174`). More explicit and avoids the latched
"which window consumes it" question, but edits more existing functions and
signatures (worse merge surface). Recommended only if upstreaming and reviewers
prefer it.

## Scope — what this does and does NOT fix

**Fixes (token available):** any launch where a valid token reaches the `zed`
process — `niri msg action spawn -- zed --add FILE:LINE`, `.desktop`/launcher
activation, or any compositor honoring xdg-activation (Mutter/KWin/Hyprland).
This is the **portable, protocol-correct** path.

**Does NOT fix on its own:**

- **Terminal-typed `zed --add` / `zed --wait`.** Terminals don't mint
  `XDG_ACTIVATION_TOKEN` for interactive commands, so there's no token to
  forward. That flow is addressed by the niri `allow-self-activation` window-rule
  (companion request) honoring Zed's *self-minted* token. This plan and that
  request are complementary: this one is "use a good token when we're handed
  one"; the niri one is "trust our stale token because the window matches a rule."
- **`--wait` via `niri msg action spawn`.** Independent breakage: niri detaches
  the child and nulls its stdio, and `niri msg` returns immediately, so `--wait`
  semantics are lost regardless of focus. Not solved here.
- **Relative paths via `niri msg action spawn`.** Another independent breakage:
  niri's spawn never sets `.current_dir()` (niri `src/utils/spawning.rs`), so the
  `zed` CLI inherits the *compositor's* working directory (≈ `$HOME` or `/`), not
  your shell's. The CLI then forwards that wrong `cwd` and relative `FILE`
  arguments resolve against it. Use absolute paths
  (`zed --add "$(realpath -- FILE):LINE"`) on the niri-spawn path. Not solved
  here; the terminal-typed path (niri allowlist) keeps the correct CWD.
- **X11.** Self-activation generally already works on X11; `DESKTOP_STARTUP_ID`
  plumbing is a possible later addition but is out of scope.

## Open questions

1. **Side-channel vs explicit API** — default to the latched side-channel for
   minimal merge surface; revisit if upstreaming.
2. **First-activate-wins** — the latched token is consumed by the next
   `activate()`. In the CLI open flow exactly one window activates, so this is
   fine; if a future flow activates multiple windows in one turn, the token goes
   to the first. Acceptable for v1; document it.
3. **Clear-on-miss** — if `activate()` self-mints (no provided token) we leave
   the latch empty; if a token was set but the open fails before any
   `activate()`, the stale token should be dropped on the next open. Set/overwrite
   the latch on every CLI open (including `None`) to avoid a leftover token from a
   previous invocation being redeemed late.
4. **New-instance path** — when the CLI starts a fresh Zed (no running instance),
   forward `XDG_ACTIVATION_TOKEN` in that child's env too; a freshly-mapped
   toplevel usually gets focus anyway, so this is secondary.

## Phasing

- **M1**: CLI field + daemon set + gpui side-channel + Wayland consume-branch.
  Verify with `niri msg action spawn -- zed --add FILE:LINE` focusing the
  existing window. (`./script/clippy`; run `cli`/`gpui` tests.)
- **M2**: clear-on-miss/overwrite semantics (open question 3), `remove_var`
  hygiene, new-instance env forwarding.
- **M3** (optional): explicit-API refactor if upstreaming; X11 `DESKTOP_STARTUP_ID`.
