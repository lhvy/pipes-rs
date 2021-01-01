mod direction;
mod pipe;
mod position;

use crossterm::{cursor, execute, style, terminal};
use pipe::Pipe;
use std::{
    io::{self, Write},
    thread,
    time::Duration,
};

fn main() -> crossterm::Result<()> {
    loop {
        let mut stdout = io::stdout();
        let mut pipe = Pipe::new()?;
        let mut ticks = 0;
        execute!(
            stdout,
            terminal::Clear(terminal::ClearType::All),
            cursor::Hide
        )?;
        while under_threshold(ticks)? {
            if pipe.tick().is_none() {
                pipe = Pipe::new()?;
            }
            execute!(stdout, cursor::MoveTo(pipe.pos.x, pipe.pos.y))?;
            execute!(stdout, style::SetForegroundColor(pipe.color))?;
            print!("{}", pipe.to_char());
            stdout.flush()?;
            ticks += 1;
            thread::sleep(Duration::from_millis(20));
        }
    }
}

fn under_threshold(ticks: u16) -> crossterm::Result<bool> {
    let (columns, rows) = terminal::size()?;
    Ok(ticks < columns * rows / 2)
}
