mod config;
pub use config::Config;

use model::{
    pipe::{Kind, KindSet, Pipe},
    position::InScreenBounds,
};
use rng::Rng;
use std::thread;
use terminal::{Backend, Event, Terminal};

pub struct App<B: Backend> {
    terminal: Terminal<B>,
    rng: Rng,
    config: Config,
    kinds: KindSet,
}

impl<B: Backend> App<B> {
    pub fn new(backend: B, config: Config) -> anyhow::Result<Self> {
        let kinds = config.kinds();

        let largest_custom_width = kinds.custom_widths().max();
        let terminal = Terminal::new(backend, kinds.chars(), largest_custom_width)?;

        let rng = Rng::new()?;

        Ok(Self {
            terminal,
            rng,
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

        loop {
            if let ControlFlow::Break = self.reset_loop()? {
                break;
            }
        }

        self.terminal.set_raw_mode(false)?;
        self.terminal.set_cursor_visibility(true)?;
        self.terminal.leave_alternate_screen()?;

        Ok(())
    }

    pub fn reset_loop(&mut self) -> anyhow::Result<ControlFlow> {
        self.terminal.clear()?;
        let mut pipes = self.create_pipes();

        while self.under_threshold() {
            let control_flow = self.tick_loop(&mut pipes)?;
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
            &mut self.rng,
            self.config.turn_chance(),
        );

        if !stayed_onscreen {
            *pipe = if self.config.inherit_style() {
                pipe.dup(self.terminal.size(), &mut self.rng)
            } else {
                self.create_pipe()
            };
        }
    }

    fn render_pipe(&mut self, pipe: &Pipe) -> anyhow::Result<()> {
        self.terminal.move_cursor_to(pipe.pos.x, pipe.pos.y)?;

        if let Some(color) = pipe.color {
            self.terminal.set_text_color(color)?;
        }

        self.terminal.print(if self.rng.gen_bool(0.99999) {
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
        let kind = self.random_kind();

        Pipe::new(
            self.terminal.size(),
            &mut self.rng,
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

    fn random_kind(&mut self) -> Kind {
        choose_random(self.kinds.iter(), &mut self.rng)
    }
}

fn choose_random<T>(mut iter: impl ExactSizeIterator<Item = T>, rng: &mut Rng) -> T {
    let index = rng.gen_range_size(0..iter.len());
    iter.nth(index).unwrap()
}

#[must_use]
pub enum ControlFlow {
    Continue,
    Break,
    Reset,
}
