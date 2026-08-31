# Tangible navigation history

@torh "Cursor history"?

We should have a way of *seeing* the navigation history. This often corresponds to paths through the code. Which should also be a concept of it's own!

Since Zed do keep track of the navigation history, it should not be *that* hard to somehow expose it as a list. Or as a multibuffer.

Having a "file history: toggle" - same as "tab-switcher: toggle" but for recently visited files could be useful.

# Code paths as first class citizen?

Stacktrace integration, but taken 5 steps further.

1. See how Sentry presents stacktraces with code context around each call-site
2. Maintain a database of the most common code paths
3. When the user navigate code they often basically trace out a code path. Make this tangible.


# Viewport history

Much of the viewstate is persisted to sqlite (not sure if it's only written on shutdown though) ... but I think it was SpaceChem where they actually used the sqlite database as the dynamic state store (during runtime)?


# Better splits?

Scenario: Often you want to view and multiple parts of the same file at the same time. Making a new editor split does work, but obstrucst everying outside that buffer as well. What I think I'd often want is to create a multi-buffer split inside that buffer/tab. Either such that there's on primary split which can be scrolled, or that both splits can be scrolled. But ideally in a way that make them "meet" instead of having both split show the same thing (or swapping the location in the file.)

Can be sort-of emulated by searching for a distinct-ish text existing around the places of interest.

# Improved multi buffers

# Expand in syntax node units

## Easy way to contract

modifier-click on the expand button is natural.

modifier-scroll ?

yes - holding eg. ctrl should keep expanding it the scroll direction instead of stopping like today

## Easy way to create new splits

Select section and run "multibuffer: hide selection" to hide the selected lines (splits the active excerpt into two smaller excerpts)

## Easy way to temporarily maximize active buffer

## Show section/excerpts boundaries in scrollbar

## Quick outline / go-to section by name

## Subtly color the linenumber/gutter using a gradient over the file length

Makes it more obvious when excerpts are far apart

## When following a symbol to it's definition, open a new multi-buffer with the original location as the first excerpt and the symbol definition as the second

# AI

## Paste symbol

Or better `@symbol ` functionality which

# Project panel

Filter panel. Typing could initiate a search/filter automatically

# Code understanding

## Inlay hints

Inlay hints showing number of references to symbols (require a *fast* language server) (or at least include the number (lazily) in the quick doc info)

## Context stack

(2025-11-27)

When "hovering" (quick doc) symbols, have an action which adds the definition (or type definition) to a context stack split. The context stack can be more or less a normal multi-buffer.


---

Any thoughts on giving the AI agent access to language server backed tools? I know this can be done using an MCP server like Serena, but running two multi-gigabyte language servers seems wasteful.



Plans for using the multibuffer (even) more? I think the multibuffer is a very cool Zed feature. I can think of many of useful things.

A couple examples:

- Open a stacktrace in a multi-buffer. Similar to the sentry stack trace view (https://sentry.io/features/stacktrace/)
- Open the edit-location history in a multibuffer
- Allow removing excerpts from a multibuffer

## A stacktrace language server

Register a `stacktrace` language (paste a trace into a scratch buffer, or open a
`*.stacktrace` file) and back it with a small language server that **wraps the
real language server** for whatever language the trace came from.

The appeal: this needs *no new Zed feature*. Everything is existing LSP surface.

- `textDocument/documentLink` — every frame becomes a link to its real location.
  Ctrl-click already works, so navigating a pasted stacktrace is free.
- `textDocument/definition` on a frame — jump to the frame's function, not just
  its line.
- `textDocument/documentSymbol` — the frame list becomes an outline, so the
  breadcrumb and outline panel work on a trace.
- `textDocument/hover` — show the source line, or the surrounding lines, inline.

**Why wrap rather than reimplement.** Turning `com.foo.Bar.baz(Bar.java:42)` or a
Python frame in `site-packages` into a real file is symbol resolution, and the
underlying server already does it via `workspace/symbol`. The wrapper's own job
is small: parse frames out of the text, ask the real server where each symbol
lives, and translate the answer back to a position in the trace buffer. It also
handles the things a plain regex cannot — source maps, JVM inner classes,
vendored paths, `<anonymous>` frames.

**Connects to the excerpt spec** (`prose/spec/2026-08-24-multibuffer-excerpt-spec.md`):
the stacktrace server resolves frames to locations; the excerpt spec is the
transport that turns a list of locations into a multibuffer. Two halves of the
Sentry-style stacktrace view above, and each is useful without the other.

Open: does the wrapper need to be a real LSP process, or is a Zed extension
enough? Also unclear how to pick *which* underlying server to wrap — the trace's
language is usually inferable from its shape, but not always.
