# pipes-rs

> An overengineered rewrite of pipes.sh in Rust

![pipes-rs preview](https://github.com/CookieCoder15/i/raw/master/pipes-rs-preview.gif)

## Installlation

Install using Homebrew.

```console
$ brew install CookieCoder15/tap/pipes-rs
```

## Configuration

pipes-rs can be configured using TOML located at `~/.config/pipes-rs/config.toml`.
The following is an example file with the default settings:

```toml
bold = true
color_mode = "ansi" # ansi, rgb or none
delay_ms = 20
inherit_style = false
kinds = ["heavy"] # heavy, light, curved, outline, emoji
num_pipes = 1
reset_threshold = 0.5 # 0.0-1.0
```

### Color Modes

| Mode | Description                                                                       |
| :--- | :-------------------------------------------------------------------------------- |
| ansi | pipe colors are randomly selected from the terminal color profile, default option |
| rgb  | pipe colors are randomly generated rgb values, unsupported in some terminals      |
| none | pipe colors will not be set and use the current terminal text color               |

### Pipe Kinds

| Kind    | Preview                    |
| :------ | :------------------------- |
| heavy   | `┃┏ ┓┛━ ┓┗┃┛┗┏ ━`          |
| light   | `│┌ ┐┘─ ┐└│┘└┌ ─`          |
| curved  | `│╭ ╮╯─ ╮╰│╯╰╭ ─`          |
| outline | `║╔ ╗╝═ ╗╚║╝╚╔ ═`          |
| emoji   | `👆👌👌👌👈👌👌👇👌👌👌👉` |

_Due to emojis having a different character width, using the emoji pipe kind along side another pipe kind can cause spacing issues._

## Options

There are also command line options that can be used to override parts of the configuration file:

| Option | Usage                                                           | Example           |
| :----- | :-------------------------------------------------------------- | :---------------- |
| `-b`   | toggles bold text                                               | `-b true`         |
| `-c`   | sets the color mode                                             | `-c rgb`          |
| `-d`   | sets the delay in ms                                            | `-d 15`           |
| `-i`   | toggles if pipes inherit style when hitting the edge            | `-i false`        |
| `-k`   | sets the kinds of pipes, each kind separated by commas          | `-k heavy,curved` |
| `-p`   | sets the number of pipes onscreen                               | `-p 5`            |
| `-r`   | sets the percentage of the screen to be filled before resetting | `-r 0.75`         |

## Acknowledgements

This project is based off of [pipes.sh](https://github.com/pipeseroni/pipes.sh).
