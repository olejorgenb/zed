# Spec: open a multibuffer from a declarative excerpt spec

Status: **draft, iterating** (Zed soft-fork feature). Pinned against `f9458db8c9`.

## Context for a fresh session

- This is a **soft fork** of Zed. `README-fork.md` at the repo root is the
  authoritative convention; the guiding constraint is a small upstream footprint,
  with the drift threshold deciding when to copy versus extract.
- **Every `file:line` here will drift.** Treat them as "find this symbol near
  here" and grep by name: `set_excerpts_for_path`, `set_excerpt_ranges_for_path`,
  `update_path_excerpts`, `build_excerpt_ranges`, `merge_excerpt_ranges`,
  `PathKey`, `ExcerptRange`, `open_paths_with_positions`, `CliRequest::Open`,
  `Editor::for_multibuffer`.
- `prose/notes/multi-buffer-guide.md` has **drifted** — its
  `set_excerpts_for_path` return type is already wrong. Read the source, not the
  guide.

## Motivation

`prose/ideas.md` already lists several features that are all the same feature
wearing different hats:

- "Open a stacktrace in a multi-buffer" (ideas.md:92) — the Sentry stacktrace view.
- "Open the edit-location history in a multibuffer" (ideas.md:93).
- "When following a symbol to its definition, open a new multi-buffer with the
  original location as the first excerpt and the symbol definition as the second"
  (ideas.md:57).

Each of these is "here is an ordered list of file ranges, show them". None of
them needs to be a bespoke Zed feature if Zed can be *handed* that list. A
declarative excerpt spec turns each one into a script that emits JSON, and
reduces the Zed-side work to a single generic reader.

That is the goal: **the substrate, not any one consumer.**

## Data model

A multibuffer is conceptually an ordered map of
`PathKey → (Entity<Buffer>, Vec<ExcerptRange<Anchor>>)`, and `ExcerptRange` is
already exactly the pair a spec needs:

```rust
struct ExcerptRange<T> {
    context: Range<T>,  // what is displayed
    primary: Range<T>,  // what is highlighted — the match
}
```

So the spec does **not** need a mode flag for "exact ranges" versus "key region
plus context". Those are two ways of filling one pair, and stating it that way
gets a third, strictly more expressive form for free:

```jsonc
// derived: give the match, let Zed pick the context
{ "primary": "48:5-50:12", "context": { "lines": 3 } }

// exact: give the displayed range, primary defaults to the whole thing
{ "context": { "range": "40-60" } }

// exact + explicit highlight
{ "context": { "range": "40-60" }, "primary": "48-50" }
```

**Why the third form matters.** When `primary == context` the excerpt has no
interior, so `editor::ContractExcerpts` becomes a silent no-op — the clamp has
nowhere to move. The same trap already exists for the split-diff base-text
excerpts, which are built via `ExcerptRange::new` (primary = context). Spec
authors writing exact ranges should be able to say what the interesting line is.

### Sketch of the whole document

```jsonc
{
  "version": 1,
  "title": "Stacktrace: NullPointerException",   // optional, names the tab
  "files": [
    {
      "path": "crates/editor/src/element.rs",
      "excerpts": [
        { "primary": "3378:1-3380:40", "context": { "lines": 4 } },
        { "context": { "range": "6825-6840" } }
      ]
    }
  ]
}
```

`files` is ordered, and that order is the display order — see PathKey below.

## Resolution semantics

Decisions, not narrative:

1. **Lines are 1-based in the spec**, converted to 0-based `Point` on the way in.
   Every external tool that will emit these specs (grep, stacktraces, LSP output)
   is 1-based. This is the most likely source of an off-by-one, so it gets stated
   once here and asserted in tests.
2. **Clamp to `max_point`, skip rather than panic.** Spec input is external and
   possibly stale — the file may have shrunk since the spec was generated. A
   range that no longer resolves is dropped, not fatal. See open questions for
   what happens when *every* range for a file is dropped.
3. **One `set_*_for_path` call per file.** `update_path_excerpts` replaces the
   path's excerpts (and removes the path entirely when handed an empty list), so
   calling the derived API and then the explicit API for the same file would wipe
   the first call. The resolver must build a single `Vec<ExcerptRange<Point>>`
   per file and make one `set_excerpt_ranges_for_path` call.
4. **Copy the context derivation.** `build_excerpt_ranges` is module-private, and
   point 3 means we cannot just call `set_excerpts_for_path` for the derived
   entries. It is ~12 trivial lines (`start_row.saturating_sub(n)`,
   `end_row + n`, clamp to `max_point`, full line at each end); copying is well
   inside `README-fork.md`'s drift threshold and keeps the change additive.
   Label the copy with the function it mirrors.
5. **Merging is automatic and not opt-out.** `merge_excerpt_ranges` fuses ranges
   that overlap *or are merely adjacent* (`end.row + 1 == start.row`), keeping
   only the **first** range's `primary`. Two nearby stacktrace frames in one file
   will become one excerpt with one highlight. This is a documented consequence,
   not a bug — but it means a spec cannot force two separate excerpts to stay
   separate if their contexts touch.
6. **`PathKey` sets display order.** `PathKey::sorted(n)` with `n` as the index in
   `files` preserves spec order; `PathKey::for_buffer` would instead sort by path.
   Spec order is almost certainly what a stacktrace wants, so default to
   `sorted(n)`.

## Entry points

Ranked cheapest-first by rebase surface, which is also the recommended order.

1. **In-app action reading a JSON spec** — from a path, or from the contents of
   the current buffer. New file plus a new action: effectively zero upstream
   footprint, and the fastest way to find out what the format actually needs.
2. **`zed://multibuffer?spec=<path>`** — near-additive in the URL router, and it
   composes with the `zed://` scheme handler already registered on this branch
   (`875808b756`). Keep the ranges *out* of the URL and point at a file; inline
   ranges get unwieldy immediately. Note the `zed:///…` parser in
   `acp_thread/src/mention.rs` is agent-internal and **not** the app's URL
   router — the real one is where `CliRequest::Open`'s `urls` are handled.
3. **A new CLI flag** — most expensive, since it edits the serialized CLI protocol
   in a shared upstream file, though a `#[serde(default)]` field on
   `CliRequest::Open` is close to additive.

### The pipeline already exists

`zed --diff a b` is this exact feature for a fixed two-file case, and is the
worked example to lift from:

- `CliRequest::Open { diff_paths, .. }` — `crates/cli/src/cli.rs`
- dispatch — `crates/zed/src/main.rs`, into `open_paths_with_positions`
  (`crates/zed/src/zed/open_listener.rs`)
- construction — `crates/git_ui/src/text_diff_view.rs`, which does
  `MultiBuffer::new(Capability::ReadWrite)`, a `set_excerpts_for_path` per file,
  then `Editor::for_multibuffer(multibuffer, Some(project.clone()), window, cx)`
  (see the call site around text_diff_view.rs:736 for the current argument list).

It also answers the awkward parts already: opening files that are not in a
worktree, and which window to target.

## Deferred

- **`"context": "syntax_node"`** — expand the context to the enclosing syntax
  node instead of a line count. The code is nearly free:
  `enclosing_syntax_node_range` exists and returns an `Option`, so it degrades
  gracefully. The trap is **timing** — tree-sitter parses asynchronously after a
  buffer opens, so a resolver running immediately may find no tree and silently
  fall back to no expansion. Doing it properly means awaiting the parse per
  buffer.

  v0 gets this post-hoc for free: open with derived context, then run the
  existing `editor::ExpandExcerptsSyntaxNode` action. The capability is not lost,
  only its declaration. Slots in later as a third `context` variant with no
  change to the v0 format.
- **Session restore.** Spec-opened items will not survive a restart. Neither do
  search-result multibuffers, so this is no worse than the status quo — but a
  spec-opened view is more likely to be something worth getting back.

## Open questions

To iterate on:

1. **Duplicate paths in one spec.** Two `files` entries with the same path are a
   `PathKey` collision, and replace semantics mean the second silently clobbers
   the first. Should the resolver pre-merge them, reject the spec, or is spec
   order meaningful enough that the same file appearing twice should genuinely
   produce two separate groups? (The last option is not expressible today.)
2. **What are relative paths relative to?** The worktree root, the spec file's
   directory, or the cwd of whatever invoked Zed. These differ in practice for a
   stacktrace generated by CI.
3. **A spec that is stale beyond clamping** — every range for a file dropped.
   Open the file with no excerpts, omit the file entirely, or fail the whole
   spec? Silent omission is friendliest and least debuggable.
4. **Versioning.** Is `"version": 1` worth carrying from the start, given the
   format will change while iterating?
5. **Read-only?** Multibuffer edits write through to the real files on save. For
   a stacktrace view that is probably desirable; for a generated report view it
   might not be. Worth a per-spec flag, or not worth the complexity.
