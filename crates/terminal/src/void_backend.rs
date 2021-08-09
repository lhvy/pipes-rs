use crate::Backend;
use crossterm::event;
use std::io;

pub struct VoidBackend;

impl io::Write for VoidBackend {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

impl Backend for VoidBackend {
    fn size(&self) -> anyhow::Result<(u16, u16)> {
        Ok((80, 24))
    }

    fn for_each_event(_f: impl FnMut(event::Event)) {}

    fn enable_raw_mode(&self) -> anyhow::Result<()> {
        Ok(())
    }

    fn disable_raw_mode(&self) -> anyhow::Result<()> {
        Ok(())
    }
}
