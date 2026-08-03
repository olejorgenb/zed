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











bar














bat












b4









sadf
