mod direction;
mod pipe;
mod position;

use crossterm::{cursor, event, execute, style, terminal};
use event::{Event, KeyCode, KeyModifiers};
use pipe::{IsOffScreen, Pipe};
use std::{
    io::{self, Write},
    thread,
    time::Duration,
};

fn main() -> crossterm::Result<()> {
    let mut stdout = io::stdout();
    terminal::enable_raw_mode()?;
    execute!(stdout, cursor::Hide)?;
    loop {
        execute!(stdout, terminal::Clear(terminal::ClearType::All))?;
        let mut pipe = Pipe::new()?;
        let mut ticks = 0;
        while under_threshold(ticks)? {
            if let Some(Event::Key(event::KeyEvent {
                code: KeyCode::Char('c'),
                modifiers: KeyModifiers::CONTROL,
            })) = get_event()?
            {
                execute!(
                    stdout,
                    terminal::Clear(terminal::ClearType::All),
                    cursor::MoveTo(0, 0),
                    cursor::Show,
                )?;
                terminal::disable_raw_mode()?;
                return Ok(());
            }
            execute!(stdout, cursor::MoveTo(pipe.pos.x, pipe.pos.y))?;
            execute!(stdout, style::SetForegroundColor(pipe.color))?;
            print!("{}", pipe.to_char());
            stdout.flush()?;
            if pipe.tick()? == IsOffScreen(true) {
                pipe = Pipe::new()?;
            }
            ticks += 1;
            thread::sleep(Duration::from_millis(20));
        }
    }
}

fn under_threshold(ticks: u16) -> crossterm::Result<bool> {
    let (columns, rows) = terminal::size()?;
    Ok(ticks < columns * rows / 2)
}

fn get_event() -> crossterm::Result<Option<Event>> {
    if event::poll(Duration::from_millis(0))? {
        event::read().map(Some)
    } else {
        Ok(None)
    }
}
