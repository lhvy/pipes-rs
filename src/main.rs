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
    let mut stdout = io::stdout();
    let mut pipe = Pipe::new()?;
    execute!(
        stdout,
        terminal::Clear(terminal::ClearType::All),
        cursor::Hide
    )?;
    loop {
        if pipe.tick().is_none() {
            pipe = Pipe::new()?;
        }
        execute!(stdout, cursor::MoveTo(pipe.pos.x, pipe.pos.y))?;
        execute!(stdout, style::SetForegroundColor(pipe.color))?;
        print!("{}", pipe.to_char());
        stdout.flush()?;
        thread::sleep(Duration::from_millis(20));
    }
}
