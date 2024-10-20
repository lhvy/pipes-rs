use clap::Parser;
use mimalloc::MiMalloc;
use model::pipe::{ColorMode, Kind, KindSet, Palette};
use pipes_rs::{App, Config};
use std::process;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// what kind of terminal coloring to use
    #[arg(short, long)]
    color_mode: Option<ColorMode>,

    /// the color palette used assign colors to pipes
    #[arg(long)]
    palette: Option<Palette>,

    /// cycle hue of pipes
    #[arg(long)]
    rainbow: Option<u8>,

    /// delay between frames in milliseconds
    #[arg(short, long)]
    delay: Option<u64>,

    /// number of frames of animation that are displayed in a second; use 0 for unlimited
    #[arg(short, long)]
    fps: Option<f32>,

    /// portion of screen covered before resetting (0.0–1.0)
    #[arg(short, long)]
    reset_threshold: Option<f32>,

    /// kinds of pipes separated by commas
    #[arg(short, long, num_args = 1.., value_delimiter = ',')]
    kinds: Option<Vec<Kind>>,

    /// toggle bold pipes
    #[arg(short, long)]
    bold: bool,

    /// toggle whether pipes should retain style after hitting the edge
    #[arg(short, long)]
    inherit_style: bool,

    /// set the number of pipes
    #[arg(short, long)]
    pipe_num: Option<u32>,

    /// chance of a pipe turning (0.0–1.0)
    #[arg(short, long)]
    turn_chance: Option<f32>,

    /// Print license
    #[arg(long)]
    license: bool,
}

fn main() -> anyhow::Result<()> {
    let mut config = Config::default();
    let cli = Cli::parse();

    config.color_mode = cli.color_mode;
    config.palette = cli.palette;
    config.rainbow = cli.rainbow;
    config.delay_ms = cli.delay;
    config.fps = cli.fps;
    config.reset_threshold = cli.reset_threshold;
    if let Some(kinds) = cli.kinds {
        config.kinds = Some(KindSet::new(kinds));
    } else {
        config.kinds = Some(KindSet::from_one(Kind::Heavy));
    }
    config.bold = Some(cli.bold);
    config.inherit_style = Some(cli.inherit_style);
    config.num_pipes = cli.pipe_num;
    config.turn_chance = cli.turn_chance;

    if cli.license {
        println!("pipes-rs is licensed under the Blue Oak Model License 1.0.0,");
        println!("the text of which you will find below.");
        println!("\n{}", include_str!("../../../LICENSE.md"));
        process::exit(0);
    }

    if cli.delay.is_some() && cli.fps.is_some() {
        anyhow::bail!("cannot specify both --delay and --fps");
    }

    let app = App::new(config)?;
    app.run()?;

    Ok(())
}
