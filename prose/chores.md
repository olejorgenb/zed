# $MODIFIER-click on expand/collapse to expand/fold all #high-pri #small

# Provide un-expand/contract excerpt actions


# Provide focus-XXX-dock-panel

Should be easy since `dock.active_panel()`


# Functionality to configure diagnostics per language-server

Eg.: should be possible to quickly toggle diagnostics from a specific language server. Prime example would be spell-check servers like Codebook.

Should be possible by direct action IMO.

Perhaps configure diagnostic level (maybe this is possible already) and style of the "squiggly lines".

# Configurable git blame gutter format

https://github.com/zed-industries/zed/issues/10771

```json
{
  ""
}
```

# Make hovers less "scared"

They run away to quickly when trying to move the mouse inside them.


# Focus last dock/pane

Possible more than one in history, but n=1 would go a long way.

# Bugs

## Python f-stringifies in wrong situation

```python
    entrypoint = EntryPointWithEnvVars(
        module=wsgi.__name__,
        environ=|"SUNSTONE_WSGI_APP_MODULE": service.environment_variables["SUNSTONE_WSGI_APP_MODULE"]},
    )
    #           ^
    #        caret -> type "{" results in
    environ={f|"SUNSTONE_WSGI_APP_MODULE": service.environment_variables["SUNSTONE_WSGI_APP_MODULE"]},
    #         ^
    #       caret

```

## Can't toggle soft-wrap in terminal view

## Wrong expand selection python

```python

if foobar:
   continue
```

Expanding selection on continue (or bare return) selects whole block, not just continue/return keyword)

### Does not display most recent commit when a detached HEAD is checked out

### Agent cancel current request on Esc-keyup

Zoom (shift-esc) while it's working -> cancelled. (hm, or perhaps the agent had stopped already :thinking:)

# Maybe

---

Swoop
https://github.com/zed-industries/zed/discussions/38606

---

Highlight all occurences across all open (visible) files
https://github.com/zed-industries/zed/issues/21549

---

https://github.com/zed-industries/zed/issues/34698

---

https://github.com/zed-industries/zed/issues/26294

---

Support markdown rendering of commit messages
https://github.com/zed-industries/zed/issues/28224
crates/markdown_preview/src/markdown_renderer.rs

Though then it should also be rendered in the full commit view, and since the markdown preview view is very gimped now I don't think that would be preferable (can't even select text IIRC)

---

Git blame follow moved code: (seems easy)
https://github.com/zed-industries/zed/discussions/42581

---

Remember last commit-message
https://github.com/zed-industries/zed/discussions/38501

---

Detect moved files in git staging panel:
https://github.com/zed-industries/zed/discussions/27044

----

https://github.com/zed-industries/zed/discussions/30696

---

Setting and shortcut for ignoring whitespaces while showing diff #27038
https://github.com/zed-industries/zed/discussions/27038

---

Support showing PR info on non-merge commits
https://github.com/zed-industries/zed/discussions/42358

---

Scroll to center in GoToDefinition if definition is not in view
https://github.com/zed-industries/zed/discussions/7651


---

Scrollbar width
https://github.com/zed-industries/zed/issues/14551

---

> Anyone know of a spell check package which easily allows you to toggle it on and off? I tried a few but didn't find a way.

I hope Zed will introduce more functionality around this in general in the future.

- Easily toggle individual LSPs / visibility of diagnostics
- Customized diagnostics visualization based on LSP (spellcheck LSP diagnostic - colored as light gray, etc)
