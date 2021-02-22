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
    let app = App::new()?;
    app.run()?;

    Ok(())
}

struct App {
    terminal: Terminal,
    config: Config,
    kinds: PresetKindSet,
    ticks: u16,
}

impl App {
    fn new() -> anyhow::Result<Self> {
        let config = read_config()?.combine(Config::from_args());
        let kinds = config.kinds();
        let terminal = Terminal::new(&kinds.chars());

        Ok(Self {
            terminal,
            config,
            kinds,
            ticks: 0,
        })
    }

    fn run(mut self) -> anyhow::Result<()> {
        self.terminal.set_raw_mode(true)?;
        self.terminal.set_cursor_visibility(false)?;
        if self.config.bold() {
            self.terminal.enable_bold()?;
        }
        while let ControlFlow::Continue = self.reset_loop()? {}

        Ok(())
    }

    fn reset_loop(&mut self) -> anyhow::Result<ControlFlow> {
        self.terminal.clear()?;
        let mut pipes = Vec::new();
        for _ in 0..self.config.num_pipes() {
            let kind = self.random_kind();
            let pipe = Pipe::new(&mut self.terminal, self.config.color_mode(), kind)?;
            pipes.push(pipe);
        }

        self.ticks = 0;

        while self.under_threshold()? {
            if let ControlFlow::Break = self.tick_loop(&mut pipes)? {
                return Ok(ControlFlow::Break);
            }
        }

        Ok(ControlFlow::Continue)
    }

    fn tick_loop(&mut self, pipes: &mut Vec<Pipe>) -> anyhow::Result<ControlFlow> {
        if self.terminal.is_ctrl_c_pressed()? {
            self.exit()?;
            return Ok(ControlFlow::Break);
        }

        for pipe in pipes {
            self.render_pipe(pipe)?;

            if pipe.tick(&mut self.terminal)? == InScreenBounds(false) {
                if self.config.inherit_style() {
                    *pipe = pipe.dup(&mut self.terminal)?;
                } else {
                    let kind = self.random_kind();
                    *pipe = Pipe::new(&mut self.terminal, self.config.color_mode(), kind)?;
                }
            }

            self.ticks += 1;
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

    fn exit(&mut self) -> anyhow::Result<()> {
        self.terminal.reset_style()?;
        self.terminal.clear()?;
        self.terminal.move_cursor_to(0, 0)?;
        self.terminal.set_cursor_visibility(true)?;
        self.terminal.set_raw_mode(false)?;

        Ok(())
    }

    fn under_threshold(&mut self) -> anyhow::Result<bool> {
        if let Some(reset_threshold) = self.config.reset_threshold() {
            let (columns, rows) = self.terminal.size()?;
            Ok(f32::from(self.ticks) < f32::from(columns) * f32::from(rows) * reset_threshold)
        } else {
            Ok(true)
        }
    }

    fn random_kind(&self) -> PresetKind {
        let PresetKindSet(ref kinds) = self.kinds;
        let index = rand::thread_rng().gen_range(0..kinds.len());

        kinds.iter().nth(index).copied().unwrap()
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

#[must_use]
enum ControlFlow {
    Continue,
    Break,
}
