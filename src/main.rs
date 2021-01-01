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

fn main() {
    let mut stdout = io::stdout();
    let mut pipe = Pipe::new();
    execute!(
        stdout,
        terminal::Clear(terminal::ClearType::All),
        cursor::Hide
    )
    .unwrap(); // TODO: Error handling properly.
    loop {
        if pipe.tick().is_none() {
            pipe = Pipe::new();
        }
        execute!(stdout, cursor::MoveTo(pipe.pos.x, pipe.pos.y)).unwrap();
        execute!(stdout, style::SetForegroundColor(pipe.color)).unwrap();
        print!("{}", pipe.dir.to_char());
        stdout.flush().unwrap();
        thread::sleep(Duration::from_millis(20));
    }
}
