mod config;
mod direction;
mod pipe;
mod position;

use anyhow::Context;
use config::Config;
use crossterm::{cursor, event, execute, style, terminal};
use etcetera::app_strategy::{AppStrategy, AppStrategyArgs, Xdg};
use event::{Event, KeyCode, KeyModifiers};
use pipe::{IsOffScreen, Pipe, PresetKind};
use rand::Rng;
use std::{
    collections::HashSet,
    fs,
    io::{self, Write},
    thread,
    time::Duration,
};

fn main() -> anyhow::Result<()> {
    let config = read_config()?;
    let kinds = config.kinds();
    let mut stdout = io::stdout();
    terminal::enable_raw_mode()?;
    execute!(stdout, cursor::Hide)?;
    if config.bold() {
        execute!(stdout, style::SetAttribute(style::Attribute::Bold))?;
    }
    loop {
        execute!(stdout, terminal::Clear(terminal::ClearType::All))?;
        let create_pipe = || Pipe::new(config.color_mode(), random_kind(&kinds));
        let mut pipe = create_pipe()?;
        let mut ticks = 0;
        while under_threshold(ticks, config.reset_threshold())? {
            if let Some(Event::Key(event::KeyEvent {
                code: KeyCode::Char('c'),
                modifiers: KeyModifiers::CONTROL,
            })) = get_event()?
            {
                execute!(
                    stdout,
                    style::SetAttribute(style::Attribute::Reset),
                    terminal::Clear(terminal::ClearType::All),
                    cursor::MoveTo(0, 0),
                    cursor::Show,
                )?;
                terminal::disable_raw_mode()?;
                return Ok(());
            }
            execute!(stdout, cursor::MoveTo(pipe.pos.x, pipe.pos.y))?;
            if let Some(color) = pipe.color {
                execute!(stdout, style::SetForegroundColor(color))?;
            }
            print!("{}", pipe.to_char());
            stdout.flush()?;
            if pipe.tick()? == IsOffScreen(true) {
                if config.inherit_style() {
                    pipe = pipe.dup()?;
                } else {
                    pipe = create_pipe()?;
                }
            }
            ticks += 1;
            thread::sleep(config.delay());
        }
    }
}

fn under_threshold(ticks: u16, reset_threshold: Option<f32>) -> crossterm::Result<bool> {
    if let Some(reset_threshold) = reset_threshold {
        let (columns, rows) = terminal::size()?;
        Ok(f32::from(ticks) < f32::from(columns) * f32::from(rows) * reset_threshold)
    } else {
        Ok(true)
    }
}

fn random_kind(kinds: &HashSet<PresetKind>) -> PresetKind {
    let index = rand::thread_rng().gen_range(0..kinds.len());
    kinds.iter().nth(index).copied().unwrap()
}

fn get_event() -> crossterm::Result<Option<Event>> {
    if event::poll(Duration::from_millis(0))? {
        event::read().map(Some)
    } else {
        Ok(None)
    }
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
