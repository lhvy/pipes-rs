mod screen;

use crossterm::event::{
    self, Event as CrosstermEvent, KeyCode, KeyEvent, KeyEventKind, KeyModifiers,
};
use crossterm::{cursor, queue, style, terminal};
use screen::Screen;
use std::io::{self, Write};
use std::num::NonZeroUsize;
use std::time::Duration;
use unicode_width::UnicodeWidthChar;

pub struct Terminal {
    screen: Screen,
    stdout: io::StdoutLock<'static>,
    max_char_width: u16,
    size: (u16, u16),
}

impl Terminal {
    pub fn new(
        stdout: io::StdoutLock<'static>,
        chars: impl Iterator<Item = char>,
        custom_width: Option<NonZeroUsize>,
    ) -> anyhow::Result<Self> {
        let max_char_width = Self::determine_max_char_width(chars, custom_width);

        let size = {
            let (width, height) = terminal::size()?;
            (width / max_char_width, height)
        };

        let screen = Screen::new(size.0 as usize, size.1 as usize);

        Ok(Self {
            screen,
            stdout,
            max_char_width,
            size,
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

    pub fn enter_alternate_screen(&mut self) -> anyhow::Result<()> {
        queue!(self.stdout, terminal::EnterAlternateScreen)?;
        Ok(())
    }

    pub fn leave_alternate_screen(&mut self) -> anyhow::Result<()> {
        queue!(self.stdout, terminal::LeaveAlternateScreen)?;
        Ok(())
    }

    pub fn set_text_color(&mut self, color: Color) -> anyhow::Result<()> {
        let color = style::Color::from(color);
        queue!(self.stdout, style::SetForegroundColor(color))?;

        Ok(())
    }

    pub fn move_cursor_to(&mut self, x: u16, y: u16) -> anyhow::Result<()> {
        queue!(self.stdout, cursor::MoveTo(x * self.max_char_width, y))?;
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
        self.stdout.write_all(c.to_string().as_bytes())?;

        Ok(())
    }

    pub fn flush(&mut self) -> anyhow::Result<()> {
        self.stdout.flush()?;
        Ok(())
    }

    pub fn get_event(&mut self) -> anyhow::Result<Option<Event>> {
        if !event::poll(Duration::ZERO)? {
            return Ok(None);
        }

        match event::read()? {
            CrosstermEvent::Resize(width, height) => {
                self.resize(width, height);
                Ok(Some(Event::Reset))
            }

            CrosstermEvent::Key(
                KeyEvent {
                    code: KeyCode::Char('c'),
                    modifiers: KeyModifiers::CONTROL,
                    kind: KeyEventKind::Press,
                    ..
                }
                | KeyEvent {
                    code: KeyCode::Char('q'),
                    kind: KeyEventKind::Press,
                    ..
                },
            ) => Ok(Some(Event::Exit)),

            CrosstermEvent::Key(KeyEvent {
                code: KeyCode::Char('r'),
                ..
            }) => Ok(Some(Event::Reset)),

            _ => Ok(None),
        }
    }

    fn resize(&mut self, width: u16, height: u16) {
        self.size = (width, height);
        self.screen.resize(width as usize, height as usize);
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
