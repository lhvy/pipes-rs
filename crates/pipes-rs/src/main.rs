mod config;

use anyhow::Context;
use config::Config;
use etcetera::app_strategy::{AppStrategy, AppStrategyArgs, Xdg};
use model::{
    pipe::{Pipe, PresetKind, PresetKindSet},
    position::InScreenBounds,
};
use rand::Rng;
use std::{fs, thread};
use structopt::StructOpt;
use terminal::{Event, Terminal};

fn main() -> anyhow::Result<()> {
    let app = App::new()?;
    app.run()?;

    Ok(())
}

struct App {
    terminal: Terminal,
    config: Config,
    kinds: PresetKindSet,
}

impl App {
    fn new() -> anyhow::Result<Self> {
        let config = read_config()?.combine(Config::from_args());
        let kinds = config.kinds();
        let terminal = Terminal::new(&kinds.chars())?;

        Ok(Self {
            terminal,
            config,
            kinds,
        })
    }

    fn run(mut self) -> anyhow::Result<()> {
        self.terminal.set_raw_mode(true)?;
        self.terminal.set_cursor_visibility(false)?;
        if self.config.bold() {
            self.terminal.enable_bold()?;
        }

        loop {
            if let ControlFlow::Break = self.reset_loop()? {
                break;
            }
        }

        Ok(())
    }

    fn reset_loop(&mut self) -> anyhow::Result<ControlFlow> {
        self.terminal.clear()?;
        let mut pipes = Vec::new();
        for _ in 0..self.config.num_pipes() {
            let kind = self.random_kind();
            let pipe = Pipe::new(
                &mut self.terminal,
                self.config.color_mode(),
                self.config.palette(),
                kind,
            );
            pipes.push(pipe);
        }

        while self.under_threshold() {
            let control_flow = self.tick_loop(&mut pipes)?;
            match control_flow {
                ControlFlow::Break | ControlFlow::Reset => return Ok(control_flow),
                _ => {}
            }
        }

        Ok(ControlFlow::Continue)
    }

    fn tick_loop(&mut self, pipes: &mut Vec<Pipe>) -> anyhow::Result<ControlFlow> {
        match self.terminal.get_event() {
            Some(Event::Resized) => return Ok(ControlFlow::Reset),
            Some(Event::CtrlCPressed) => {
                self.reset_terminal()?;
                return Ok(ControlFlow::Break);
            }
            None => {}
        }

        for pipe in pipes {
            self.render_pipe(pipe)?;

            if pipe.tick(&mut self.terminal, self.config.turn_chance()) == InScreenBounds(false) {
                if self.config.inherit_style() {
                    *pipe = pipe.dup(&mut self.terminal);
                } else {
                    let kind = self.random_kind();
                    *pipe = Pipe::new(
                        &mut self.terminal,
                        self.config.color_mode(),
                        self.config.palette(),
                        kind,
                    );
                }
            }
        }

        self.terminal.flush()?;
        thread::sleep(self.config.delay());

        Ok(ControlFlow::Continue)
    }

    fn render_pipe(&mut self, pipe: &mut Pipe) -> anyhow::Result<()> {
        self.terminal.move_cursor_to(pipe.pos.x, pipe.pos.y)?;

        if let Some(color) = pipe.color {
            self.terminal.set_text_color(color)?;
        }

        self.terminal.print(pipe.to_char())?;

        Ok(())
    }

    fn reset_terminal(&mut self) -> anyhow::Result<()> {
        self.terminal.reset_style()?;
        self.terminal.clear()?;
        self.terminal.move_cursor_to(0, 0)?;
        self.terminal.set_cursor_visibility(true)?;
        self.terminal.set_raw_mode(false)?;

        Ok(())
    }

    fn under_threshold(&self) -> bool {
        if let Some(reset_threshold) = self.config.reset_threshold() {
            self.terminal.portion_covered() < reset_threshold
        } else {
            true
        }
    }

    fn random_kind(&self) -> PresetKind {
        let PresetKindSet(ref kinds) = self.kinds;
        *choose_random(kinds.iter())
    }
}

fn choose_random<T>(mut iter: impl ExactSizeIterator<Item = T>) -> T {
    let index = rand::thread_rng().gen_range(0..iter.len());
    iter.nth(index).unwrap()
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
    Reset,
}
