use mimalloc::MiMalloc;
use model::pipe::{ColorMode, Palette};
use pipes_rs::{App, Config};
use std::{env, process};

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

fn main() -> anyhow::Result<()> {
    let mut config = Config::read()?;
    parse_args(&mut config);
    config.validate()?;

    let app = App::new(config)?;
    app.run()?;

    Ok(())
}

fn parse_args(config: &mut Config) {
    let args: Vec<_> = env::args().skip(1).collect();
    let mut args_i = args.iter();

    while let Some(arg) = args_i.next() {
        match arg.as_str() {
            "--license" => {
                if args.len() != 1 {
                    eprintln!("error: provided arguments other than --license");
                    process::exit(1);
                }

                println!("pipes-rs is licensed under the Blue Oak Model License 1.0.0,");
                println!("the text of which you will find below.");
                println!("\n{}", include_str!("../../../LICENSE.md"));
                process::exit(0);
            }

            "--version" | "-V" => {
                if args.len() != 1 {
                    eprintln!("error: provided arguments other than --version");
                    process::exit(1);
                }

                println!("pipes-rs {}", env!("CARGO_PKG_VERSION"));
                process::exit(0);
            }

            "--help" => {
                println!("{}", include_str!("usage"));
                process::exit(0);
            }

            _ => {}
        }

        let (option, value) = arg.split_once('=').unwrap_or_else(|| match args_i.next() {
            Some(value) => (arg, value),
            None => required_value(arg),
        });

        match option {
            "--color-mode" | "-c" => {
                config.color_mode = match value {
                    "ansi" => Some(ColorMode::Ansi),
                    "rgb" => Some(ColorMode::Rgb),
                    "none" => Some(ColorMode::None),
                    _ => invalid_value(option, value, "“ansi”, “rgb” or “none”"),
                }
            }

            "--palette" => {
                config.palette = match value {
                    "default" => Some(Palette::Default),
                    "darker" => Some(Palette::Darker),
                    "pastel" => Some(Palette::Pastel),
                    "matrix" => Some(Palette::Matrix),
                    _ => invalid_value(option, value, "“default”, “darker”, “pastel” or “matrix”"),
                }
            }

            "--rainbow" => {
                config.rainbow = match value.parse() {
                    Ok(v) => Some(v),
                    Err(_) => invalid_value(option, value, "an integer between 0 and 255"),
                }
            }

            "--delay" | "-d" => {
                config.delay_ms = match value.parse() {
                    Ok(v) => Some(v),
                    Err(_) => invalid_value(option, value, "a positive integer"),
                }
            }

            "--fps" | "-f" => {
                config.fps = match value.parse() {
                    Ok(v) => Some(v),
                    Err(_) => invalid_value(option, value, "a number"),
                }
            }

            "--reset-threshold" | "-r" => {
                config.reset_threshold = match value.parse() {
                    Ok(v) => Some(v),
                    Err(_) => invalid_value(option, value, "a number"),
                }
            }

            "--kinds" | "-k" => {
                config.kinds = match value.parse() {
                    Ok(v) => Some(v),
                    Err(_) => invalid_value(option, value, "kinds of pipes separated by commas"),
                }
            }

            "--bold" | "-b" => {
                config.bold = match value.parse() {
                    Ok(v) => Some(v),
                    Err(_) => invalid_value(option, value, "“true” or “false”"),
                }
            }

            "--inherit-style" | "-i" => {
                config.inherit_style = match value.parse() {
                    Ok(v) => Some(v),
                    Err(_) => invalid_value(option, value, "“true” or “false”"),
                }
            }

            "--pipe-num" | "-p" => {
                config.num_pipes = match value.parse() {
                    Ok(v) => Some(v),
                    Err(_) => invalid_value(option, value, "a positive integer"),
                }
            }

            "--turn-chance" | "-t" => {
                config.turn_chance = match value.parse() {
                    Ok(v) => Some(v),
                    Err(_) => invalid_value(option, value, "a number"),
                }
            }

            _ => {
                eprintln!("error: unrecognized option {option}");
                eprintln!("see --help");
                process::exit(1);
            }
        }
    }
}

fn required_value(option: &str) -> ! {
    eprintln!("error: a value is required for {option} but none was supplied");
    eprintln!("see --help");
    process::exit(1);
}

fn invalid_value(option: &str, actual: &str, expected: &str) -> ! {
    eprintln!("error: invalid value “{actual}” for {option}");
    eprint!("       expected {expected}");
    eprintln!("\nsee --help");
    process::exit(1);
}
