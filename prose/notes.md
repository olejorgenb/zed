# Settings hierarchy

## Confusing which settings can be overriden/set as a project setting

https://github.com/zed-industries/zed/discussions/6564#discussioncomment-10574800

# Language servers

A principled configuration framework for language servers!

Ie.: per language server configuration of:
- Diagnostics level to show (`diagnostics_max_severity`)
- Which features to use. This is probably often possible (and maybe better) to configure when starting the server, but I think it make sense to just provide configuration of how Zed responds and which servers it use for various features.

# Commands / Actions

`zed --dump-all-action`


# To investigate

dev::ToggleInspector
