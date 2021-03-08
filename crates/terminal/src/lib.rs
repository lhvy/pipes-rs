mod screen;

use crossterm::{
    cursor,
    event::{self, Event as CrosstermEvent, KeyCode, KeyEvent, KeyModifiers},
    queue, style, terminal,
};
use screen::Screen;
use std::{
    io::{self, Write},
    thread,
};
use unicode_width::UnicodeWidthChar;

pub struct Terminal {
    screen: Screen,
    stdout: io::Stdout,
    max_char_width: u16,
    size: (u16, u16),
    events_rx: flume::Receiver<EventWithData>,
}

impl Terminal {
    pub fn new(
        chars: impl Iterator<Item = char>,
        custom_width: Option<usize>,
    ) -> anyhow::Result<Self> {
        let max_char_width = Self::determine_max_char_width(chars, custom_width);

        let size = {
            let (width, height) = terminal::size()?;
            (width / max_char_width, height)
        };

        let screen = Screen::new(size.0 as usize, size.1 as usize);

        let (events_tx, events_rx) = flume::unbounded();

        thread::spawn(move || loop {
            match event::read() {
                Ok(CrosstermEvent::Resize(width, height)) => events_tx
                    .send(EventWithData::Resized { width, height })
                    .unwrap(),

                Ok(CrosstermEvent::Key(KeyEvent {
                    code: KeyCode::Char('c'),
                    modifiers: KeyModifiers::CONTROL,
                }))
                | Ok(CrosstermEvent::Key(KeyEvent {
                    code: KeyCode::Char('q'),
                    ..
                })) => events_tx.send(EventWithData::Exit).unwrap(),

                Ok(CrosstermEvent::Key(KeyEvent {
                    code: KeyCode::Char('r'),
                    ..
                })) => events_tx.send(EventWithData::Reset).unwrap(),

                Ok(_) => {} // ignore all other events

                // ignore errors because not updating the size
                // isn’t a fatal issue
                Err(_) => {}
            }
        });

        Ok(Self {
            screen,
            stdout: io::stdout(),
            max_char_width,
            size,
            events_rx,
        })
    }

    fn determine_max_char_width(
        chars: impl Iterator<Item = char>,
        custom_width: Option<usize>,
    ) -> u16 {
        let max_char_width = chars.map(|c| c.width().unwrap() as u16).max().unwrap();

        if let Some(custom_width) = custom_width {
            max_char_width.max(custom_width as u16)
        } else {
            max_char_width
        }
    }

    pub fn enable_bold(&mut self) -> anyhow::Result<()> {
        queue!(self.stdout, style::SetAttribute(style::Attribute::Bold))?;
        Ok(())
    }

    pub fn reset_style(&mut self) -> anyhow::Result<()> {
        queue!(self.stdout, style::SetAttribute(style::Attribute::Reset))?;
        Ok(())
    }

    pub fn set_cursor_visibility(&mut self, visible: bool) -> anyhow::Result<()> {
        if visible {
            queue!(self.stdout, cursor::Show)?;
        } else {
            queue!(self.stdout, cursor::Hide)?;
        }

        Ok(())
    }

    pub fn clear(&mut self) -> anyhow::Result<()> {
        queue!(self.stdout, terminal::Clear(terminal::ClearType::All))?;
        self.screen.clear();

        Ok(())
    }

    pub fn set_raw_mode(&self, enabled: bool) -> anyhow::Result<()> {
        if enabled {
            terminal::enable_raw_mode()?;
        } else {
            terminal::disable_raw_mode()?;
        }

        Ok(())
    }

    pub fn set_text_color(&mut self, color: Color) -> anyhow::Result<()> {
        let color = style::Color::from(color);
        queue!(self.stdout, style::SetForegroundColor(color))?;

        Ok(())
    }

    pub fn move_cursor_to(&mut self, x: u16, y: u16) -> anyhow::Result<()> {
        let max_char_width = self.max_char_width;
        queue!(self.stdout, cursor::MoveTo(x * max_char_width, y))?;
        self.screen.move_cursor_to(x as usize, y as usize);

        Ok(())
    }

    pub fn portion_covered(&self) -> f32 {
        self.screen.portion_covered()
    }

    pub fn size(&self) -> (u16, u16) {
        self.size
    }

    pub fn print(&mut self, c: char) -> anyhow::Result<()> {
        self.screen.print(c);
        self.stdout.write_all(c.to_string().as_bytes())?;

        Ok(())
    }

    pub fn flush(&mut self) -> anyhow::Result<()> {
        self.stdout.flush()?;
        Ok(())
    }

    pub fn get_event(&mut self) -> Option<Event> {
        match self.events_rx.try_recv().ok() {
            Some(EventWithData::Exit) => Some(Event::Exit),
            Some(EventWithData::Reset) => Some(Event::Reset),
            Some(EventWithData::Resized { width, height }) => {
                self.resize(width, height);
                Some(Event::Reset)
            }
            None => None,
        }
    }

    fn resize(&mut self, width: u16, height: u16) {
        self.size = (width, height);
        self.screen = Screen::new(width as usize, height as usize);
    }
}

#[derive(Clone, Copy)]
pub enum Color {
    Red,
    DarkRed,
    Green,
    DarkGreen,
    Yellow,
    DarkYellow,
    Blue,
    DarkBlue,
    Magenta,
    DarkMagenta,
    Cyan,
    DarkCyan,
    Rgb { r: u8, g: u8, b: u8 },
}

impl From<Color> for style::Color {
    fn from(color: Color) -> Self {
        match color {
            Color::Red => Self::Red,
            Color::DarkRed => Self::DarkRed,
            Color::Green => Self::Green,
            Color::DarkGreen => Self::DarkGreen,
            Color::Yellow => Self::Yellow,
            Color::DarkYellow => Self::DarkYellow,
            Color::Blue => Self::Blue,
            Color::DarkBlue => Self::DarkBlue,
            Color::Magenta => Self::Magenta,
            Color::DarkMagenta => Self::DarkMagenta,
            Color::Cyan => Self::Cyan,
            Color::DarkCyan => Self::DarkCyan,
            Color::Rgb { r, g, b } => Self::Rgb { r, g, b },
        }
    }
}

pub enum Event {
    Exit,
    Reset,
}

enum EventWithData {
    Exit,
    Reset,
    Resized { width: u16, height: u16 },
}
