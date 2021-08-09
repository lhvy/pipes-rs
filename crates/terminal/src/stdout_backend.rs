use crate::Backend;
use crossterm::{event, terminal};
use std::io::{self, Write};

pub struct StdoutBackend<'a> {
    stdout: io::StdoutLock<'a>,
}

impl<'a> StdoutBackend<'a> {
    pub fn new(stdout: io::StdoutLock<'a>) -> Self {
        Self { stdout }
    }
}

impl Write for StdoutBackend<'_> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.stdout.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.stdout.flush()
    }
}

impl Backend for StdoutBackend<'_> {
    fn size(&self) -> anyhow::Result<(u16, u16)> {
        Ok(terminal::size()?)
    }

    fn for_each_event(mut f: impl FnMut(event::Event)) {
        loop {
            if let Ok(event) = event::read() {
                f(event);
            }
        }
    }

    fn enable_raw_mode(&self) -> anyhow::Result<()> {
        terminal::enable_raw_mode()?;
        Ok(())
    }

    fn disable_raw_mode(&self) -> anyhow::Result<()> {
        terminal::disable_raw_mode()?;
        Ok(())
    }
}
