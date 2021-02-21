use anyhow::Context;
use etcetera::app_strategy::{AppStrategy, AppStrategyArgs, Xdg};
use model::{
    config::Config,
    pipe::{Pipe, PresetKind, PresetKindSet},
    position::InScreenBounds,
};
use rand::Rng;
use std::{fs, thread};
use structopt::StructOpt;
use terminal::Terminal;

fn main() -> anyhow::Result<()> {
    let config = read_config()?.combine(Config::from_args());
    let kinds = config.kinds();
    let mut terminal = Terminal::new(&kinds.chars());
    terminal.set_raw_mode(true)?;
    terminal.set_cursor_visibility(false)?;
    if config.bold() {
        terminal.enable_bold()?;
    }
    while let ControlFlow::Continue = reset_loop(&mut terminal, &config, &kinds)? {}
    Ok(())
}

fn reset_loop(
    terminal: &mut Terminal,
    config: &Config,
    kinds: &PresetKindSet,
) -> anyhow::Result<ControlFlow> {
    terminal.clear()?;
    let mut pipes = Vec::new();
    for _ in 0..config.num_pipes() {
        let pipe = Pipe::new(terminal, config.color_mode(), random_kind(&kinds))?;
        pipes.push(pipe);
    }

    let mut ticks = 0;
    while under_threshold(terminal, ticks, config.reset_threshold())? {
        if let ControlFlow::Break = tick_loop(terminal, &mut pipes, config, kinds, &mut ticks)? {
            return Ok(ControlFlow::Break);
        }
    }

    Ok(ControlFlow::Continue)
}

fn tick_loop(
    terminal: &mut Terminal,
    pipes: &mut Vec<Pipe>,
    config: &Config,
    kinds: &PresetKindSet,
    ticks: &mut u16,
) -> anyhow::Result<ControlFlow> {
    if terminal.is_ctrl_c_pressed()? {
        exit(terminal)?;
        return Ok(ControlFlow::Break);
    }

    for pipe in pipes {
        terminal.move_cursor_to(pipe.pos.x, pipe.pos.y)?;
        if let Some(color) = pipe.color {
            terminal.set_text_color(color)?;
        }
        terminal.print(pipe.to_char())?;
        if pipe.tick(terminal)? == InScreenBounds(false) {
            if config.inherit_style() {
                *pipe = pipe.dup(terminal)?;
            } else {
                *pipe = Pipe::new(terminal, config.color_mode(), random_kind(&kinds))?;
            }
        }
        *ticks += 1;
    }
    terminal.flush()?;
    thread::sleep(config.delay());

    Ok(ControlFlow::Continue)
}

fn exit(terminal: &mut Terminal) -> anyhow::Result<()> {
    terminal.reset_style()?;
    terminal.clear()?;
    terminal.move_cursor_to(0, 0)?;
    terminal.set_cursor_visibility(true)?;
    terminal.set_raw_mode(false)?;

    Ok(())
}

fn under_threshold(
    terminal: &mut Terminal,
    ticks: u16,
    reset_threshold: Option<f32>,
) -> anyhow::Result<bool> {
    if let Some(reset_threshold) = reset_threshold {
        let (columns, rows) = terminal.size()?;
        Ok(f32::from(ticks) < f32::from(columns) * f32::from(rows) * reset_threshold)
    } else {
        Ok(true)
    }
}

fn random_kind(PresetKindSet(kinds): &PresetKindSet) -> PresetKind {
    let index = rand::thread_rng().gen_range(0..kinds.len());
    kinds.iter().nth(index).copied().unwrap()
}

fn read_config() -> anyhow::Result<Config> {
    let path = Xdg::new(AppStrategyArgs {
        top_level_domain: "io.github".to_string(),
        author: "CookieCoder15".to_string(),
        app_name: "pipes-rs".to_string(),
    })?
    .in_config_dir("config.toml");
    if path.exists() {
        let contents = fs::read_to_string(path)?;
        Ok(toml::from_str(&contents).context("failed to read config")?)
    } else {
        Ok(Config::default())
    }
}

#[must_use]
enum ControlFlow {
    Continue,
    Break,
}
