# pipes-rs

[![Bors enabled](https://bors.tech/images/badge_small.svg)](https://app.bors.tech/repositories/32076) ![GitHub Actions CI status](https://github.com/lhvy/pipes-rs/actions/workflows/ci.yaml/badge.svg)

> An over-engineered rewrite of pipes.sh in Rust

![pipes-rs preview](https://github.com/lhvy/i/raw/master/pipes-rs-preview.gif)

## Installation

Install on any platform using Cargo:

```
cargo install --git https://github.com/lhvy/pipes-rs
```

Alternatively for macOS, install via Homebrew:

```
brew install lhvy/tap/pipes-rs
```

Alternatively for Windows, install via Scoop:

```
scoop bucket add extras
scoop install pipes-rs
```

#### Manual Download

Download compiled binaries from [releases](https://github.com/lhvy/pipes-rs/releases/latest).

## Windows Font Issues

There have been reports that some characters pipes-rs uses are missing on Windows, which causes them to appear as [tofu](https://en.wikipedia.org/wiki/Noto_fonts#Etymology). If you experience this issue, try using a font with a large character set, such as [Noto Mono](https://www.google.com/get/noto/).

## Keybindings

- <kbd>r</kbd>: reset the screen
- <kbd>q</kbd> or <kbd>^C</kbd>: exit the program

## Configuration

pipes-rs can be configured using TOML located at `~/.config/pipes-rs/config.toml`.
The following is an example file with the default settings:

```toml
bold = true
color_mode = "ansi" # ansi, rgb or none
palette = "default" # default, darker, pastel or matrix
rainbow = 0 # 0-255
delay_ms = 20
inherit_style = false
kinds = ["heavy"] # heavy, light, curved, knobby, emoji, outline, dots, blocks, sus
num_pipes = 1
reset_threshold = 0.5 # 0.0–1.0
turn_chance = 0.15 # 0.0–1.0
```

### Color Modes

| Mode   | Description                                                                       |
| :----- | :-------------------------------------------------------------------------------- |
| `ansi` | pipe colors are randomly selected from the terminal color profile, default option |
| `rgb`  | pipe colors are randomly generated rgb values, unsupported in some terminals      |
| `none` | pipe colors will not be set and use the current terminal text color               |

### Palettes

| Palette   | Description                                                      |
| :-------- | :--------------------------------------------------------------- |
| `default` | bright colors – good on dark backgrounds, default option         |
| `darker`  | darker colors – good on light backgrounds                        |
| `pastel`  | pastel colors – good on dark backgrounds                         |
| `matrix`  | colors based on [Matrix digital rain] – good on dark backgrounds |

### Pipe Kinds

| Kind      | Preview                   |
| :-------- | :------------------------ |
| `heavy`   | `┃ ┃ ━ ━ ┏ ┓ ┗ ┛`         |
| `light`   | `│ │ ─ ─ ┌ ┐ └ ┘`         |
| `curved`  | `│ │ ─ ─ ╭ ╮ ╰ ╯`         |
| `knobby`  | `╽ ╿ ╼ ╾ ┎ ┒ ┖ ┚`         |
| `emoji`   | `👆 👇 👈 👉 👌 👌 👌 👌` |
| `outline` | `║ ║ ═ ═ ╔ ╗ ╚ ╝`         |
| `dots`    | `• • • • • • • •`         |
| `blocks`  | `█ █ ▀ ▀ █ █ ▀ ▀`         |
| `sus`     | `ඞ ඞ ඞ ඞ ඞ ඞ ඞ ඞ`         |

_Due to emojis having a different character width, using the emoji pipe kind along side another pipe kind can cause spacing issues._

## Options

There are also command line options that can be used to override parts of the configuration file:

| Option      | Usage                                                                             | Example            |
| :---------- | :-------------------------------------------------------------------------------- | :----------------- |
| `-b`        | toggles bold text                                                                 | `-b true`          |
| `-c`        | sets the color mode                                                               | `-c rgb`           |
| `-d`        | sets the delay in ms                                                              | `-d 15`            |
| `-i`        | toggles if pipes inherit style when hitting the edge                              | `-i false`         |
| `-k`        | sets the kinds of pipes, each kind separated by commas                            | `-k heavy,curved`  |
| `-p`        | sets the number of pipes on screen                                                | `-p 5`             |
| `-r`        | sets the percentage of the screen to be filled before resetting                   | `-r 0.75`          |
| `-t`        | chance of a pipe turning each frame                                               | `-t 0.15`          |
| `--palette` | sets the color palette, RGB mode only                                             | `--palette pastel` |
| `--rainbow` | sets the number of degrees per frame to shift the hue of each pipe, RGB mode only | `--rainbow 5`      |

## Credits

### Contributors

pipes-rs is maintained by [lhvy](https://github.com/lhvy) and [lunacookies](https://github.com/lunacookies); any other contributions via PRs are welcome! Forks and modifications are implicitly licensed under the Blue Oak Model License 1.0.0. Please credit the above contributors and pipes.sh when making forks or other derivative works.

### Inspiration

This project is based off of [pipes.sh](https://github.com/pipeseroni/pipes.sh).

[matrix digital rain]: https://en.wikipedia.org/wiki/Matrix_digital_rain
