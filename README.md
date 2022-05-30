# Firefox Tabs Pop Launcher Plugin

**DISCLAIMER**: Work in Progress

Plugin that lists firefox tabs in [pop os launcher](https://github.com/pop-os/launcher).

## Installation

```console
```

Or if you prefer, you can do the same the script does, **manually**:

- Download the latest binary and `plugin.ron` from [releases](https://github.com/rcastill/pop-launcher-firefox-tabs/releases)
- Create directory `mkdir -p $HOME/.local/share/pop-launcher/plugins/firefox-tabs`
- Place binary and `plugin.ron` inside folder
- Rename binary from `pop-launcher-firefox-tabs` to `firefox-tabs`
- Give execution permissions to binary `chmod u+x $HOME/.local/share/pop-launcher/plugins/firefox-tabs/firefox-tabs`

## Limitations

- Because of a limitation with [firefox-rs](https://github.com/rcastill/firefox-rs), at the moment of writing, you must install [focusTab](https://addons.mozilla.org/en-US/firefox/addon/focus_tab/) firefox extension.
- In order to list tabs, `firefox-rs` uses [this method](https://superuser.com/questions/269443/list-open-firefox-tabs-from-the-command-line), which means the results may be outdated for a couple of seconds (until firefox writes to the backup file)

## TODO

- [x] Improve search implementation (1st iteration)
- [x] Favicon as item icon?
- [x] Installation script
- [ ] Publish firefox-rs and this plugin