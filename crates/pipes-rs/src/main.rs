use anyhow::Context;
use etcetera::app_strategy::{AppStrategy, AppStrategyArgs, Xdg};
use model::{
    config::Config,
    pipe::{IsOffScreen, Pipe, PresetKind},
};
use rand::Rng;
use std::{collections::HashSet, fs, thread};
use terminal::Terminal;

fn main() -> anyhow::Result<()> {
    let config = read_config()?;
    let kinds = config.kinds();
    let mut terminal = Terminal::default();
    terminal.set_raw_mode(true)?;
    terminal.set_cursor_visibility(false)?;
    if config.bold() {
        terminal.enable_bold()?;
    }
    loop {
        terminal.clear()?;
        let mut pipes = Vec::new();
        for _ in 0..config.num_pipes() {
            let pipe = Pipe::new(&mut terminal, config.color_mode(), random_kind(&kinds))?;
            pipes.push(pipe);
        }
        let mut ticks = 0;
        while under_threshold(&mut terminal, ticks, config.reset_threshold())? {
            if terminal.is_ctrl_c_pressed()? {
                terminal.reset_style()?;
                terminal.clear()?;
                terminal.move_cursor_to(0, 0)?;
                terminal.set_cursor_visibility(true)?;
                terminal.set_raw_mode(false)?;
                return Ok(());
            }
            for pipe in &mut pipes {
                terminal.move_cursor_to(pipe.pos.x, pipe.pos.y)?;
                if let Some(color) = pipe.color {
                    terminal.set_text_color(color)?;
                }
                print!("{}", pipe.to_char());
                if pipe.tick(&mut terminal)? == IsOffScreen(true) {
                    if config.inherit_style() {
                        *pipe = pipe.dup(&mut terminal)?;
                    } else {
                        *pipe = Pipe::new(&mut terminal, config.color_mode(), random_kind(&kinds))?;
                    }
                }
                ticks += 1;
            }
            terminal.flush()?;
            thread::sleep(config.delay());
        }
    }
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

fn random_kind(kinds: &HashSet<PresetKind>) -> PresetKind {
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
