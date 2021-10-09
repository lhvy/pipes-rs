mod screen;
mod stdout_backend;
mod void_backend;

pub use stdout_backend::StdoutBackend;
pub use void_backend::VoidBackend;

use crossterm::{
    cursor,
    event::{Event as CrosstermEvent, KeyCode, KeyEvent, KeyModifiers},
    queue, style, terminal,
};
use screen::Screen;
use std::{io::Write, num::NonZeroUsize, thread};
use unicode_width::UnicodeWidthChar;

pub struct Terminal<B: Backend> {
    screen: Screen,
    backend: B,
    max_char_width: u16,
    size: (u16, u16),
    events_rx: flume::Receiver<EventWithData>,
}

impl<B: Backend> Terminal<B> {
    pub fn new(
        backend: B,
        chars: impl Iterator<Item = char>,
        custom_width: Option<NonZeroUsize>,
    ) -> anyhow::Result<Self> {
        let max_char_width = Self::determine_max_char_width(chars, custom_width);

        let size = {
            let (width, height) = backend.size()?;
            (width / max_char_width, height)
        };

        let screen = Screen::new(size.0 as usize, size.1 as usize);

        let (events_tx, events_rx) = flume::unbounded();

        thread::spawn(move || {
            B::for_each_event(|event| {
                match event {
                    CrosstermEvent::Resize(width, height) => events_tx
                        .send(EventWithData::Resized { width, height })
                        .unwrap(),

                    CrosstermEvent::Key(
                        KeyEvent {
                            code: KeyCode::Char('c'),
                            modifiers: KeyModifiers::CONTROL,
                        }
                        | KeyEvent {
                            code: KeyCode::Char('q'),
                            ..
                        },
                    ) => events_tx.send(EventWithData::Exit).unwrap(),

                    CrosstermEvent::Key(KeyEvent {
                        code: KeyCode::Char('r'),
                        ..
                    }) => events_tx.send(EventWithData::Reset).unwrap(),

                    _ => {} // ignore all other events
                }
            });
        });

        Ok(Self {
            screen,
            backend,
            max_char_width,
            size,
            events_rx,
        })
    }

    fn determine_max_char_width(
        chars: impl Iterator<Item = char>,
        custom_width: Option<NonZeroUsize>,
    ) -> u16 {
        let max_char_width = chars.map(|c| c.width().unwrap() as u16).max().unwrap();

        match custom_width {
            Some(custom_width) => max_char_width.max(custom_width.get() as u16),
            None => max_char_width,
        }
    }

    pub fn enable_bold(&mut self) -> anyhow::Result<()> {
        queue!(self.backend, style::SetAttribute(style::Attribute::Bold))?;
        Ok(())
    }

    pub fn reset_style(&mut self) -> anyhow::Result<()> {
        queue!(self.backend, style::SetAttribute(style::Attribute::Reset))?;
        Ok(())
    }

    pub fn set_cursor_visibility(&mut self, visible: bool) -> anyhow::Result<()> {
        if visible {
            queue!(self.backend, cursor::Show)?;
        } else {
            queue!(self.backend, cursor::Hide)?;
        }

        Ok(())
    }

    pub fn clear(&mut self) -> anyhow::Result<()> {
        queue!(self.backend, terminal::Clear(terminal::ClearType::All))?;
        self.screen.clear();

        Ok(())
    }

    pub fn set_raw_mode(&self, enabled: bool) -> anyhow::Result<()> {
        if enabled {
            self.backend.enable_raw_mode()?;
        } else {
            self.backend.disable_raw_mode()?;
        }

        Ok(())
    }

    pub fn enter_alternate_screen(&mut self) -> anyhow::Result<()> {
        queue!(self.backend, terminal::EnterAlternateScreen)?;
        Ok(())
    }

    pub fn leave_alternate_screen(&mut self) -> anyhow::Result<()> {
        queue!(self.backend, terminal::LeaveAlternateScreen)?;
        Ok(())
    }

    pub fn set_text_color(&mut self, color: Color) -> anyhow::Result<()> {
        let color = style::Color::from(color);
        queue!(self.backend, style::SetForegroundColor(color))?;

        Ok(())
    }

    #[inline(always)]
    pub fn move_cursor_to(&mut self, x: u16, y: u16) -> anyhow::Result<()> {
        let max_char_width = self.max_char_width;
        queue!(self.backend, cursor::MoveTo(x * max_char_width, y))?;
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
        self.screen.print();
        self.backend.write_all(c.to_string().as_bytes())?;

        Ok(())
    }

    pub fn flush(&mut self) -> anyhow::Result<()> {
        self.backend.flush()?;
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

pub trait Backend: Write {
    fn size(&self) -> anyhow::Result<(u16, u16)>;
    fn for_each_event(f: impl FnMut(CrosstermEvent));
    fn enable_raw_mode(&self) -> anyhow::Result<()>;
    fn disable_raw_mode(&self) -> anyhow::Result<()>;
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
