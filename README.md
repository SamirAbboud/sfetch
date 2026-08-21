# sfetch

A fast and configurable system information fetcher written in Rust.

sfetch displays system information using configurable fetch functions, layouts, colors, and ASCII logos.

## Features

- System and hardware information
- Configurable layouts
- Fetch functions with optional arguments
- Automatic distro logos
- Custom ASCII logos
- 16-color ANSI support
- Configurable text and function colors
- Custom logo positions
- Built-in default configuration
- Custom configuration files

# Installation

## From source

Clone the repository:

```bash
git clone https://github.com/SamirAbboud/sfetch.git
cd sfetch
```

Install sfetch with Cargo

```bash
cargo install --path .
```

This builds the project in release mode and installs the sfetch executable to:

```bash
~/.cargo/bin/sfetch
```

## Usage

Run sfetch with the default configuration:

```bash
sfetch
```

Use a specific configuration file:

```bash
sfetch --config ~/.config/sfetch/config.toml
```

Publish the default configuration:

```bash
sfetch --publish-config
```

If no `--config` argument is provided, sfetch looks for:

```text
~/.config/sfetch/config.toml
```

If that file does not exist, the built-in defaults are used.

---

# Configuration

sfetch uses TOML configuration files.

## Layout

The `layout` option controls what information is displayed.

Fetch functions are inserted between `{}`:

```toml
layout = """
Hardware & OS
  >  Distro: {distro()}
  >  Kernel: {kernel()}
  > 󰌢 Laptop: {model(true)}
  > 󰍹 Monitor: {monitor()}
  >  GPU: {gpu()}
  >  CPU: {cpu()}
  >  Memory: {memory()}
  >  Disk: {disk()}
  >  Driver: {gpu_driver()}

Software & Misc
  >  WM: {wm()}
  >  Shell: {shell()}
  >  Terminal: {terminal()}
  > 󰔚 Uptime: {uptime()}
  >  Packages: {packages()}
  >  Theme: {gtk_theme()}
  >  Icons: {icon_theme()}
  >  Font: {gtk_font()}

          {colors()}
"""
```

Empty lines in the layout are preserved.

## Function arguments

Arguments are optional. Each function has its own defaults.

For example:

```text
{distro()}
```

uses the default arguments, while:

```text
{distro(false)}
```

overrides the first argument.

You do not need to pass every argument unless you want to change a default.

---

# Fetch Functions

## `distro`

Displays the Linux distribution.

```text
{distro()}
```

Arguments:

```text
{distro(architecture)}
```

- `architecture`: `true` includes the architecture.
- `false` omits it.

Default:

```text
true
```

Example:

```text
{distro()}
{distro(false)}
```

---

## `kernel`

Displays the kernel version.

```text
{kernel()}
```

Arguments:

```text
{kernel(small)}
```

- `small`: use the short kernel representation.
- `false`: use the normal/full representation.

Default:

```text
false
```

Example:

```text
{kernel()}
{kernel(true)}
```

---

## `model`

Displays the computer model.

```text
{model()}
```

Arguments:

```text
{model(version)}
```

- `version`: include the model version.

Default:

```text
false
```

Example:

```text
{model()}
{model(true)}
```

---

## `wm`

Displays the current window manager.

```text
{wm()}
```

Arguments:

```text
{wm(protocol)}
```

- `protocol`: include the display protocol.

Default:

```text
true
```

Example:

```text
{wm()}
{wm(false)}
```

---

## `shell`

Displays the current shell.

```text
{shell()}
```

Arguments:

```text
{shell(version)}
```

- `version`: include the shell version.

Default:

```text
true
```

Example:

```text
{shell()}
{shell(false)}
```

---

## `gpu`

Displays GPU information.

```text
{gpu()}
```

Arguments:

```text
{gpu(full_name, colorize)}
```

- `full_name`: display the full GPU name.
- `colorize`: colorize individual GPU entries.

Defaults:

```text
full_name = true
colorize = false
```

Example:

```text
{gpu()}
{gpu(false, true)}
```

---

## `gpu_driver`

Displays the GPU driver.

```text
{gpu_driver()}
```

Arguments:

```text
{gpu_driver(single_driver)}
```

- `single_driver`: request a single driver instead of the normal combined output.

Default:

```text
false
```

Example:

```text
{gpu_driver()}
{gpu_driver(true)}
```

---

## `cpu`

Displays CPU information.

```text
{cpu()}
```

Arguments:

```text
{cpu(round_to, full_name, colorize)}
```

- `round_to`: number of decimal places for the CPU frequency.
- `full_name`: display the full CPU name.
- `colorize`: colorize CPU information.

Defaults:

```text
round_to = 2
full_name = false
colorize = false
```

Example:

```text
{cpu()}
{cpu(2, true, false)}
```

---

## `memory`

Displays memory usage.

```text
{memory()}
```

Arguments:

```text
{memory(gib, round_to, colorize)}
```

- `gib`: display values in GiB instead of MiB.
- `round_to`: number of decimal places.
- `colorize`: colorize memory percent usage.

Defaults:

```text
gib = true
round_to = 2
colorize = true
```

Example:

```text
{memory()}
{memory(true, 2, true)}
```

---

## `disk`

Displays filesystem usage.

```text
{disk()}
```

Arguments:

```text
{disk(path, colorize, file_system, percent, round_to)}
```

- `path`: filesystem path.
- `colorize`: colorize disk usage percent.
- `file_system`: display the filesystem type.
- `percent`: display the usage percentage.
- `round_to`: number of decimal places.

Defaults:

```text
path = "/"
colorize = true
file_system = true
percent = true
round_to = 2
```

Example:

```text
{disk()}
{disk("/", true, true, true, 2)}
```

---

## `monitor`

Displays monitor information.

```text
{monitor()}
```

This function currently does not require arguments.

---

## `uptime`

Displays system uptime.

```text
{uptime()}
```

Arguments:

```text
{uptime(up, length)}
```

- `up`: controls the uptime output mode.
- `length`: output length.

Available lengths:

```text
full
medium
short
```

Default length:

```text
full
```

Example:

```text
{uptime()}
{uptime(false, medium)}
```

---

## `hostname`

Displays the hostname.

```text
{hostname()}
```

No arguments are required.

---

## `terminal`

Displays the terminal emulator.

```text
{terminal()}
```

No arguments are required.

---

## `packages`

Displays installed package information.

```text
{packages()}
```

No arguments are required.

---

## `gtk_theme`

Displays the current GTK theme.

```text
{gtk_theme()}
```

No arguments are required.

---

## `icon_theme`

Displays the current icon theme.

```text
{icon_theme()}
```

No arguments are required.

---

## `gtk_font`

Displays the current GTK font.

```text
{gtk_font()}
```

No arguments are required.

---

## `cursor_theme`

Displays the current cursor theme.

```text
{cursor_theme()}
```

No arguments are required.

---

## `colors`

Displays the terminal color palette.

```text
{colors()}
```

Arguments:

```text
{colors(background, character, normal_only)}
```

- `background`: use background colors.
- `character`: character displayed for each color.
- `normal_only`: display only normal colors.

Defaults:

```text
background = true
character = "  "
normal_only = true
```

Example:

```text
{colors()}
{colors(false, "  ", true)}
```

---

# Colors

## `colorize_functions`

Controls whether fetch-function output is automatically colorized.

```toml
colorize_functions = true
```

When `true`, function results use `functions_color`.

When `false`, function results use the surrounding text color.

This allows individual lines or sections to be manually colorized through the layout.

## `text_color`

Controls the color used for normal layout text.

```toml
text_color = "logo"
```

`"logo"` uses the main color of the selected logo.

A numeric color can also be specified.

```toml
text_color = "4"
```

The available colors are numbered from `0` to `15`.

## `functions_color`

Controls the default color used for fetch-function output when `colorize_functions` is enabled.

```toml
functions_color = 7
```

Colors are numbered from `0` to `15`.

---

# Logos

## Automatic logo

The default logo setting is:

```toml
logo = "auto"
```

`auto` automatically selects a logo based on the detected distribution.

A specific distro logo can also be selected:

```toml
logo = "arch"
```

## Custom logo

Custom logos can be defined using a `logo = {}` table:

```toml
logo = {
    logo = """{4}
     █████╗ ██████╗
    ██╔══██╗██╔══██╗
""",
    main_color = 4
}

The logo supports color placeholders from `{0}` through `{15}`.

For example:

```text
{4}
```

inserts ANSI color 4.

`main_color` is also used when:

```toml
text_color = "logo"
```

---

# Logo Options

## `logo_info_whitespace`

Controls the number of spaces between the logo and system information.

```toml
logo_info_whitespace = 5
```

## `lstrip_info`

Removes leading whitespace from every layout line.

```toml
lstrip_info = false
```

## `print_logo`

Controls whether the logo is displayed.

```toml
print_logo = true
```

Set it to `false` to display only the system information.

## `logo_position`

Controls where the logo is placed.

```toml
logo_position = "left"
```

Available values:

```text
left
top
right
bottom
```

---

# Example Configuration

```toml
layout = """
Hardware & OS
  >  Distro: {distro()}
  >  Kernel: {kernel()}
  > 󰌢 Laptop: {model(true)}
  > 󰍹 Monitor: {monitor()}
  >  GPU: {gpu()}
  >  CPU: {cpu()}
  >  Memory: {memory()}
  >  Disk: {disk()}
  >  Driver: {gpu_driver()}

Software & Misc
  >  WM: {wm()}
  >  Shell: {shell()}
  >  Terminal: {terminal()}
  > 󰔚 Uptime: {uptime()}
  >  Packages: {packages()}
  >  Theme: {gtk_theme()}
  >  Icons: {icon_theme()}
  >  Font: {gtk_font()}

          {colors()}
"""

colorize_functions = true
text_color = "logo"
functions_color = 7

logo = "auto"

logo_info_whitespace = 5
lstrip_info = false
print_logo = true
logo_position = "left"
```

---

# Configuration Resolution

sfetch resolves configuration in this order:

1. A path explicitly passed with `--config`
2. `~/.config/sfetch/config.toml`
3. Built-in defaults

Example:

```bash
sfetch --config ~/.config/sfetch/minimal.toml
```

---

# Development

Build:

```bash
cargo build
```

Run:

```bash
cargo run
```

Run with a configuration:

```bash
cargo run -- --config ~/.config/sfetch/config.toml
```

Check:

```bash
cargo check
```

Test:

```bash
cargo test
```

Format:

```bash
cargo fmt
```

Lint:

```bash
cargo clippy
```
