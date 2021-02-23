use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent, KeyModifiers},
    queue, style, terminal,
};
use parking_lot::Mutex;
use std::{
    fmt::Display,
    io::{self, Write},
    sync::Arc,
    thread,
};
use unicode_width::UnicodeWidthChar;

pub struct Terminal {
    stdout: io::Stdout,
    max_char_width: u16,
    size: Arc<Mutex<(u16, u16)>>,
    ctrl_c_rx: flume::Receiver<CtrlCPresed>,
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

impl Terminal {
    pub fn new(chars: &[char]) -> anyhow::Result<Self> {
        let max_char_width = chars
            .iter()
            .map(|c| c.width().unwrap() as u16)
            .max()
            .unwrap();

        let size = {
            let (width, height) = terminal::size()?;
            (width / max_char_width, height)
        };

        let size = Arc::new(Mutex::new(size));

        let (ctrl_c_tx, ctrl_c_rx) = flume::unbounded();

        thread::spawn({
            let size = Arc::clone(&size);

            move || {
                loop {
                    match event::read() {
                        Ok(Event::Resize(x, y)) => *size.lock() = (x, y),

                        Ok(Event::Key(KeyEvent {
                            code: KeyCode::Char('c'),
                            modifiers: KeyModifiers::CONTROL,
                        })) => ctrl_c_tx.send(CtrlCPresed).unwrap(),

                        Ok(_) => {} // ignore all other events

                        // ignore errors because not updating the size
                        // isn’t a fatal issue
                        Err(_) => {}
                    }
                }
            }
        });

        Ok(Self {
            stdout: io::stdout(),
            max_char_width,
            size,
            ctrl_c_rx,
        })
    }

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
        let color = style::Color::from(color);
        queue!(self.stdout, style::SetForegroundColor(color))?;
        Ok(())
    }

    pub fn move_cursor_to(&mut self, x: u16, y: u16) -> anyhow::Result<()> {
        let max_char_width = self.max_char_width;
        queue!(self.stdout, cursor::MoveTo(x * max_char_width, y))?;
        Ok(())
    }

    pub fn size(&self) -> (u16, u16) {
        *self.size.lock()
    }

    pub fn print(&mut self, s: impl Display) -> anyhow::Result<()> {
        self.stdout.write_all(format!("{}", s).as_bytes())?;
        Ok(())
    }

    pub fn flush(&mut self) -> anyhow::Result<()> {
        self.stdout.flush()?;
        Ok(())
    }

    pub fn is_ctrl_c_pressed(&self) -> bool {
        self.ctrl_c_rx.try_recv().is_ok()
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

struct CtrlCPresed;
