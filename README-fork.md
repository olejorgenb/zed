# This checkout is a soft fork of Zed

This is a personal soft fork of `zed-industries/zed`. Most work here lives on
long-lived branches that are rebased onto a fast-moving upstream many times and
may never merge. That single fact drives every convention below.

`CLAUDE.md`, `AGENTS.md` and `.rules` in this repo are **upstream Zed's**
contributor guidance, unmodified. They are written for PR-bound work. Where they
conflict with this document, this document wins for branch work — and this
document explicitly does *not* apply to anything headed for an upstream PR.

## Premises

1. **The work may never land upstream.** Merging is a hope, not a plan. A branch
   that is pleasant to rebase for two years beats a branch that would pass Zed
   review once.
2. **Rebase cost is recurring; review cost is one-off.** Every line this fork
   changes in a file upstream also edits is a conflict paid again on each rebase.
   Every line it merely *adds* is nearly free forever.
3. **Not all conflicts are bad.** Trivial conflicts are fine. Some conflicts are
   *informative*: they surface real drift-caused bugs that a naive textual rebase
   would otherwise hide.
4. **Duplication hides drift.** A copied helper will silently keep the old
   behaviour when upstream fixes the original, and no rebase will ever tell you.
   This is the honest cost of premise 2's remedy — it is a trade, not a free win.
5. **The balance usually favours a small footprint,** because the conflict cost is
   certain and recurring while the drift cost is speculative. "Usually" is not
   "always": see the drift threshold below.

## Topology

| Ref | Role |
|---|---|
| `upstream` | `git@github.com:zed-industries/zed.git` — the real Zed. |
| `origin` | `git@github.com:olejorgenb/zed.git` — the personal GitHub fork, used to publish and back up branches. Note this is *not* where `main` comes from. |
| `main` | Pure mirror of `upstream/main`, and tracks it directly. Never diverges. Never commit here. |
| `ole` | The long-lived integration branch. All personal features land here, stacked on `main`, rebased periodically. |
| feature branches | Branch off `ole` (e.g. `expand-by-syntax`), one feature each, folded back into `ole`. |
| `ole-jj` | Archive remote (`/home/ole/contrib/zed-jj`) of an abandoned `jj` experiment. Its `salvage/*` branches are old WIP being re-landed one at a time. |
| `prose/` | Notes, ideas, chores, session logs. Lives only on `ole`; upstream has no such directory. |
| `script/find-release-commit` | Personal tooling, same deal. |

Salvaging an `ole-jj/salvage/*` branch usually means **re-implementing** it on the
current base rather than cherry-picking: upstream has typically moved too far.
Delete the remote ref once the work is re-landed.

## Writing code here

- **Additive over refactoring.** Prefer copying a small helper into new code over
  extracting a shared one, so the existing upstream function stays byte-identical.
  A *deletion* inside a function upstream actively edits is the worst kind of
  footprint: it conflicts on every rebase and has to be re-derived each time.
- **The drift threshold.** Ask: *would I notice the drift by reading my copy?*
  At ~8 readable lines, yes — copy it. At ~70 lines, or if the logic is genuinely
  tricky, no — keep the shared extraction and accept the conflicts, precisely so
  drift surfaces.
- **Label deliberate duplication.** A comment naming the function the copy mirrors,
  so drift is one `git diff` away at rebase time, plus a note in the commit
  message so nobody "cleans it up" later.
- **Rank the touches you can't avoid**, worst first:
  - `Cargo.lock` / `crates/*/Cargo.toml` — among the most conflict-prone files in
    the repo. Adding a crate dependency to a shared crate is rarely worth it;
    prefer doing the work in a crate that already depends on it.
  - trait signature changes — ripple to every impl, but usually only a handful.
  - `assets/settings/default.json` — upstream edits it constantly, but the hunks
    are tiny and conflicts are always trivial.
  - new files, new enum variants, appended functions and tests — effectively free.
- **Settings go JSON-only.** Add to `settings_content` + resolved settings, skip
  `settings_ui`/`page_data.rs` wiring and skip `docs/`. Smallest upstream
  footprint, and the docs would be wrong the moment this is rebased.
- **Squash commits that rewrite the same hunks.** Carried separately, every future
  rebase makes you resolve the same regions once per commit.

## Reviewing code here

Review criteria are correctness, style, simplicity, **and rebasability**. A review
of a change on this fork should include a *rebase surface* section: which files
outside the feature's own crate are touched, and how much each one will hurt on
the next rebase. Recommending the upstream-idiomatic refactor without pricing its
rebase cost is a review that missed the point.

## The inversion

Everything above is the opposite of what a Zed upstream reviewer would tell you.
If a change is going to become a PR, drop these rules and follow `AGENTS.md` /
`.rules` instead: factor out the shared helper, wire up the settings UI, write the
docs.
