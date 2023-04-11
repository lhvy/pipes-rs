mod config;
pub use config::Config;

use model::pipe::{KindSet, Pipe};
use model::position::InScreenBounds;
use std::{io, thread};
use terminal::{Event, Terminal};

pub struct App {
    terminal: Terminal,
    config: Config,
    kinds: KindSet,
}

impl App {
    pub fn new(config: Config) -> anyhow::Result<Self> {
        let kinds = config.kinds();

        let stdout = io::stdout().lock();
        let largest_custom_width = kinds.custom_widths().max();
        let terminal = Terminal::new(stdout, kinds.chars(), largest_custom_width)?;

        Ok(Self {
            terminal,
            config,
            kinds,
        })
    }

    pub fn run(mut self) -> anyhow::Result<()> {
        self.terminal.enter_alternate_screen()?;
        self.terminal.set_raw_mode(true)?;
        self.terminal.set_cursor_visibility(false)?;
        if self.config.bold() {
            self.terminal.enable_bold()?;
        }

        let mut pipes = self.create_pipes();

        loop {
            if let ControlFlow::Break = self.reset_loop(&mut pipes)? {
                break;
            }
        }

        self.terminal.set_raw_mode(false)?;
        self.terminal.set_cursor_visibility(true)?;
        self.terminal.leave_alternate_screen()?;

        Ok(())
    }

    pub fn reset_loop(&mut self, pipes: &mut Vec<Pipe>) -> anyhow::Result<ControlFlow> {
        self.terminal.clear()?;

        for pipe in &mut *pipes {
            *pipe = self.create_pipe();
        }

        while self.under_threshold() {
            let control_flow = self.tick_loop(pipes)?;
            match control_flow {
                ControlFlow::Break | ControlFlow::Reset => return Ok(control_flow),
                ControlFlow::Continue => {}
            }
        }

        Ok(ControlFlow::Continue)
    }

    pub fn tick_loop(&mut self, pipes: &mut Vec<Pipe>) -> anyhow::Result<ControlFlow> {
        match self.terminal.get_event() {
            Some(Event::Reset) => return Ok(ControlFlow::Reset),
            Some(Event::Exit) => return Ok(ControlFlow::Break),
            None => {}
        }

        for pipe in pipes {
            self.render_pipe(pipe)?;
            self.tick_pipe(pipe);
        }

        self.terminal.flush()?;
        thread::sleep(self.config.delay());

        Ok(ControlFlow::Continue)
    }

    fn tick_pipe(&mut self, pipe: &mut Pipe) {
        let InScreenBounds(stayed_onscreen) = pipe.tick(
            self.terminal.size(),
            self.config.turn_chance(),
            self.config.rainbow(),
        );

        if !stayed_onscreen {
            *pipe = if self.config.inherit_style() {
                pipe.dup(self.terminal.size())
            } else {
                self.create_pipe()
            };
        }
    }

    fn render_pipe(&mut self, pipe: &Pipe) -> anyhow::Result<()> {
        self.terminal
            .move_cursor_to(pipe.position.x, pipe.position.y)?;

        if let Some(color) = pipe.color {
            self.terminal.set_text_color(color.terminal)?;
        }

        self.terminal.print(if rng::gen_bool(0.99999) {
            pipe.to_char()
        } else {
            '🦀'
        })?;

        Ok(())
    }

    pub fn create_pipes(&mut self) -> Vec<Pipe> {
        (0..self.config.num_pipes())
            .map(|_| self.create_pipe())
            .collect()
    }

    fn create_pipe(&mut self) -> Pipe {
        let kind = self.kinds.choose_random();

        Pipe::new(
            self.terminal.size(),
            self.config.color_mode(),
            self.config.palette(),
            kind,
        )
    }

    fn under_threshold(&self) -> bool {
        match self.config.reset_threshold() {
            Some(reset_threshold) => self.terminal.portion_covered() < reset_threshold,
            None => true,
        }
    }
}

#[must_use]
pub enum ControlFlow {
    Continue,
    Break,
    Reset,
}
