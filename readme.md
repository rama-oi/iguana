# Coqui

![preview](https://raw.githubusercontent.com/rama-oi/coqui/main/assets/screenshots/0.png)

A fast, keyboard-driven TUI application launcher.

## Features

- **Inline calculator**: type a math expression (`12 * 4`, `(3 + 4) / 2`) and the list collapses to a single result row. Press Enter to copy it straight to your clipboard.
- **Keyboard-first navigation**: `arrow keys` to move through the list, `Enter` to launch or select, `Esc` to back out.
- **Built-in settings**: always available as the last item in the list, no separate hotkey needed.
- **Theming**: switch between bundled themes (Catppuccin, Dracula, Tokyo Night, and more) live from the settings screen `ctrl + s`.
- **Custom Lists**: toggle between your default list and your system list with `ctrl + l`.

> In order to create a new element for your custom list create an entry in the `coqui_config.toml` like the following:
```toml
[[command]]
label = "Jaiba"
cmd = "alacritty -e jaiba"
description = "TUI Password manager"
```
or this for a separator
```toml
[[command]]
label = "-"
```

## Installation

You need a small window-manager configuration to make Coqui's terminal float at a fixed size.

> I'm using `Alacritty` in these examples, but feel free to test it with your preferred terminal.

### Sway

Add the following to `~/.config/sway/config`:

```ini
for_window [app_id="floating-terminal"] floating enable, resize set width 400px height 300px, move position center
````

Call Coqui with:

```sh
alacritty --class floating-terminal -e coqui
```

### i3

> ⚠️ **Untested:** If this works for you, please open an issue so I can validate the configuration.

Add the following to `~/.config/i3/config`:

```ini
for_window [class="floating-terminal"] floating enable, resize set 400 px 300 px, move position center
```

Call Coqui with:

```sh
alacritty --class floating-terminal -e coqui
```

### Hyprland

> ⚠️ **Untested:** If this works for you, please open an issue so I can validate the configuration.

For Hyprland 0.55+, add the following to `~/.config/hypr/hyprland.lua`:

```lua
hl.window_rule({
    match = {
        class = "floating-terminal"
    },
    float = true,
    size = "400 300",
    center = true
})
```

For Hyprland 0.54 or older, add the following to `~/.config/hypr/hyprland.conf`:

```ini
windowrule = float, class:^(floating-terminal)$
windowrule = size 400 300, class:^(floating-terminal)$
windowrule = center, class:^(floating-terminal)$
```

Call Coqui with:

```sh
alacritty --class floating-terminal -e coqui
```

### Openbox

> ⚠️ **Untested:** If this works for you, please open an issue so I can validate the configuration. 

> **Hint:** If `class` doesn't work, try `name` instead.

Add the following to `~/.config/openbox/rc.xml` inside the `<applications>` section:
```xml
<application class="floating-terminal">
    <decor>no</decor>
    <focus>yes</focus>
    <position force="yes">
        <x>center</x>
        <y>center</y>
    </position>
    <size>
        <width>400</width>
        <height>300</height>
    </size>
</application>
```
Call Coqui with:
```sh
alacritty --class floating-terminal -e coqui
```