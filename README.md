# Firefox Tabs Pop Launcher Plugin

Plugin that lists firefox tabs in [pop os launcher](https://github.com/pop-os/launcher).



https://user-images.githubusercontent.com/8534987/170946002-a22634e0-7658-4ab7-97ea-6411a82adb39.mp4



## Installation

First, you have to install firefox extension [focusTab](https://addons.mozilla.org/en-US/firefox/addon/focus_tab/) for this plugin to be able to focus on the selected task. See [Limitations](#limitations).

Then execute the following command in your terminal:

> **What is this?** This will automatically download the plugin and install it under `$HOME/.local/share/pop-launcher/plugins/firefox-tabs`. You can inspect the installation script [here](https://github.com/rcastill/pop-launcher-firefox-tabs/blob/master/scripts/install.sh).

```console
curl --proto '=https' -sSf https://raw.githubusercontent.com/rcastill/pop-launcher-firefox-tabs/master/scripts/install.sh | bash
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
