use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent, KeyModifiers},
    queue, style, terminal,
};
use std::{io::{self, Write}, time::Duration};

pub struct Terminal {
    stdout: io::Stdout,
}

macro_rules! gen_terminal_method {
    ($name: ident, $command: expr) => {
        pub fn $name(&mut self) -> anyhow::Result<()> {
            queue!(self.stdout, $command)?;
            Ok(())
        }
    };
}

macro_rules! gen_terminal_method_bool {
    ($name: ident, $param: ident, $true_command: expr, $false_command: expr) => {
        pub fn $name(&mut self, $param: bool) -> anyhow::Result<()> {
            if $param {
                queue!(self.stdout, $true_command)?;
            } else {
                queue!(self.stdout, $false_command)?;
            }
            Ok(())
        }
    };
}

impl Default for Terminal {
    fn default() -> Self {
        Self {
            stdout: io::stdout(),
        }
    }
}

impl Terminal {
    gen_terminal_method!(clear, terminal::Clear(terminal::ClearType::All));
    gen_terminal_method!(enable_bold, style::SetAttribute(style::Attribute::Bold));
    gen_terminal_method!(reset_style, style::SetAttribute(style::Attribute::Reset));

    gen_terminal_method_bool!(set_cursor_visibility, visible, cursor::Show, cursor::Hide);

    pub fn set_raw_mode(&mut self, enabled: bool) -> anyhow::Result<()> {
        if enabled {
            terminal::enable_raw_mode()?;
        } else {
            terminal::disable_raw_mode()?;
        }
        Ok(())
    }

    pub fn set_text_color(&mut self, color: Color) -> anyhow::Result<()> {
        let color = match color {
            Color::Red => style::Color::Red,
            Color::DarkRed => style::Color::DarkRed,
            Color::Green => style::Color::Green,
            Color::DarkGreen => style::Color::DarkGreen,
            Color::Yellow => style::Color::Yellow,
            Color::DarkYellow => style::Color::DarkYellow,
            Color::Blue => style::Color::Blue,
            Color::DarkBlue => style::Color::DarkBlue,
            Color::Magenta => style::Color::Magenta,
            Color::DarkMagenta => style::Color::DarkMagenta,
            Color::Cyan => style::Color::Cyan,
            Color::DarkCyan => style::Color::DarkCyan,
            Color::Rgb { r, g, b } => style::Color::Rgb { r, g, b },
        };
        queue!(self.stdout, style::SetForegroundColor(color))?;
        Ok(())
    }

    pub fn move_cursor_to(&mut self, x: u16, y: u16) -> anyhow::Result<()> {
        queue!(self.stdout, cursor::MoveTo(x, y))?;
        Ok(())
    }

    pub fn size(&self) -> anyhow::Result<(u16, u16)> {
        Ok(terminal::size()?)
    }

    pub fn flush(&mut self) -> anyhow::Result<()> {
        self.stdout.flush()?;
        Ok(())
    }

    pub fn is_ctrl_c_pressed(&self) -> anyhow::Result<bool> {
        Ok(Some(Event::Key(KeyEvent {
            code: KeyCode::Char('c'),
            modifiers: KeyModifiers::CONTROL,
        })) == self.get_event()?)
    }

    fn get_event(&self) -> anyhow::Result<Option<Event>> {
        if event::poll(Duration::from_millis(0))? {
            Ok(event::read().map(Some)?)
        } else {
            Ok(None)
        }
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
