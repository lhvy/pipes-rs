mod config;

use config::Config;
use mimalloc::MiMalloc;
use model::{
    pipe::{Pipe, PresetKind, PresetKindSet},
    position::InScreenBounds,
};
use rng::Rng;
use std::{io, thread};
use structopt::StructOpt;
use terminal::{Event, Terminal};

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

fn main() -> anyhow::Result<()> {
    let stdout = io::stdout();
    let app = App::new(stdout.lock())?;
    app.run()?;

    Ok(())
}

struct App<'a> {
    terminal: Terminal<'a>,
    rng: Rng,
    config: Config,
    kinds: PresetKindSet,
}

impl<'a> App<'a> {
    fn new(stdout: io::StdoutLock<'a>) -> anyhow::Result<Self> {
        let config = Config::read()?.combine(Config::from_args());
        config.validate()?;

        let kinds = config.kinds();

        let largest_custom_width = kinds.custom_widths().max();
        let terminal = Terminal::new(stdout, kinds.chars(), largest_custom_width)?;

        let rng = Rng::new()?;

        Ok(Self {
            terminal,
            rng,
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

    fn tick_loop(&mut self, pipes: &mut Vec<Pipe>) -> anyhow::Result<ControlFlow> {
        match self.terminal.get_event() {
            Some(Event::Reset) => return Ok(ControlFlow::Reset),
            Some(Event::Exit) => {
                self.reset_terminal()?;
                return Ok(ControlFlow::Break);
            }
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

    fn reset_terminal(&mut self) -> anyhow::Result<()> {
        self.terminal.reset_style()?;
        self.terminal.clear()?;
        self.terminal.move_cursor_to(0, 0)?;
        self.terminal.set_cursor_visibility(true)?;
        self.terminal.set_raw_mode(false)?;

        Ok(())
    }

    fn create_pipes(&mut self) -> Vec<Pipe> {
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
        if let Some(reset_threshold) = self.config.reset_threshold() {
            self.terminal.portion_covered() < reset_threshold
        } else {
            true
        }
    }

    fn random_kind(&mut self) -> PresetKind {
        let PresetKindSet(ref kinds) = self.kinds;
        *choose_random(kinds.iter(), &mut self.rng)
    }
}

fn choose_random<T>(mut iter: impl ExactSizeIterator<Item = T>, rng: &mut Rng) -> T {
    let index = rng.gen_range_size(0..iter.len());
    iter.nth(index).unwrap()
}

#[must_use]
enum ControlFlow {
    Continue,
    Break,
    Reset,
}
