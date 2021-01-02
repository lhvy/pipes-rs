use crate::config::ColorMode;
use crate::direction::Direction;
use crate::position::Position;
use crossterm::{style, terminal};
use rand::Rng;

pub(crate) struct Pipe {
    pub(crate) dir: Direction,
    pub(crate) pos: Position,
    pub(crate) color: Option<style::Color>,
    kind: Kind,
    prev_dir: Direction,
    just_turned: bool,
}

impl Pipe {
    pub(crate) fn new(color_mode: ColorMode, preset_kind: PresetKind) -> crossterm::Result<Self> {
        Self::new_raw(gen_random_color(color_mode), preset_kind.kind())
    }

    fn new_raw(color: Option<style::Color>, kind: Kind) -> crossterm::Result<Self> {
        let (dir, pos) = Self::gen_rand_dir_and_pos()?;
        Ok(Self {
            dir,
            pos,
            color,
            kind,
            prev_dir: dir,
            just_turned: false,
        })
    }

    fn gen_rand_dir_and_pos() -> crossterm::Result<(Direction, Position)> {
        let (columns, rows) = terminal::size()?;
        let dir = rand::thread_rng().gen();
        let pos = match dir {
            Direction::Up => Position {
                x: rand::thread_rng().gen_range(0..columns),
                y: rows - 1,
            },
            Direction::Down => Position {
                x: rand::thread_rng().gen_range(0..columns),
                y: 0,
            },
            Direction::Left => Position {
                x: columns - 1,
                y: rand::thread_rng().gen_range(0..rows),
            },
            Direction::Right => Position {
                x: 0,
                y: rand::thread_rng().gen_range(0..rows),
            },
        };
        Ok((dir, pos))
    }

    pub(crate) fn dup(&self) -> crossterm::Result<Self> {
        Self::new_raw(self.color, self.kind)
    }

    pub(crate) fn tick(&mut self) -> crossterm::Result<IsOffScreen> {
        if !self.pos.can_move_in(self.dir)? {
            return Ok(IsOffScreen(true));
        }
        self.prev_dir = self.dir;
        self.just_turned = self.dir.maybe_turn();
        Ok(IsOffScreen(false))
    }

    pub(crate) fn to_char(&self) -> char {
        if self.just_turned {
            match (self.prev_dir, self.dir) {
                (Direction::Up, Direction::Left) | (Direction::Right, Direction::Down) => {
                    self.kind.top_right
                }
                (Direction::Up, Direction::Right) | (Direction::Left, Direction::Down) => {
                    self.kind.top_left
                }
                (Direction::Down, Direction::Left) | (Direction::Right, Direction::Up) => {
                    self.kind.bottom_right
                }
                (Direction::Down, Direction::Right) | (Direction::Left, Direction::Up) => {
                    self.kind.bottom_left
                }
                _ => unreachable!(),
            }
        } else {
            match self.dir {
                Direction::Up => self.kind.up,
                Direction::Down => self.kind.down,
                Direction::Left => self.kind.left,
                Direction::Right => self.kind.right,
            }
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize, Eq, PartialEq, Hash, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub(crate) enum PresetKind {
    Heavy,
    Light,
    Curved,
    // Emoji,
    Outline,
}

#[derive(Clone, Copy)]
struct Kind {
    up: char,
    down: char,
    left: char,
    right: char,
    top_left: char,
    top_right: char,
    bottom_left: char,
    bottom_right: char,
}

impl PresetKind {
    const HEAVY: Kind = Kind {
        up: '┃',
        down: '┃',
        left: '━',
        right: '━',
        top_left: '┏',
        top_right: '┓',
        bottom_left: '┗',
        bottom_right: '┛',
    };

    const LIGHT: Kind = Kind {
        up: '│',
        down: '│',
        left: '─',
        right: '─',
        top_left: '┌',
        top_right: '┐',
        bottom_left: '└',
        bottom_right: '┘',
    };

    const CURVED: Kind = Kind {
        up: '│',
        down: '│',
        left: '─',
        right: '─',
        top_left: '╭',
        top_right: '╮',
        bottom_left: '╰',
        bottom_right: '╯',
    };

    /*
    const EMOJI: Kind = Kind {
        up: '👆',
        down: '👇',
        left: '👈',
        right: '👉',
        top_left: '👌',
        top_right: '👌',
        bottom_left: '👌',
        bottom_right: '👌',
    };
    */

    const OUTLINE: Kind = Kind {
        up: '║',
        down: '║',
        left: '═',
        right: '═',
        top_left: '╔',
        top_right: '╗',
        bottom_left: '╚',
        bottom_right: '╝',
    };

    fn kind(&self) -> Kind {
        match self {
            Self::Heavy => Self::HEAVY,
            Self::Light => Self::LIGHT,
            Self::Curved => Self::CURVED,
            // Self::Emoji => Self::EMOJI,
            Self::Outline => Self::OUTLINE,
        }
    }
}

fn gen_random_color(color_mode: ColorMode) -> Option<style::Color> {
    match color_mode {
        ColorMode::Ansi => {
            let num = rand::thread_rng().gen_range(0..=11);
            Some(match num {
                0 => style::Color::Red,
                1 => style::Color::DarkRed,
                2 => style::Color::Green,
                3 => style::Color::DarkGreen,
                4 => style::Color::Yellow,
                5 => style::Color::DarkYellow,
                6 => style::Color::Blue,
                7 => style::Color::DarkBlue,
                8 => style::Color::Magenta,
                9 => style::Color::DarkMagenta,
                10 => style::Color::Cyan,
                11 => style::Color::DarkCyan,
                _ => unreachable!(),
            })
        }
        ColorMode::Rgb => {
            let hue = rand::thread_rng().gen_range(0.0..=360.0);
            let lch = color_space::Lch {
                l: 75.0,
                c: 75.0,
                h: hue,
            };
            let color_space::Rgb { r, g, b } = color_space::Rgb::from(lch);
            Some(style::Color::Rgb {
                r: r as u8,
                g: g as u8,
                b: b as u8,
            })
        }
        ColorMode::None => None,
    }
}

#[derive(PartialEq)]
pub(crate) struct IsOffScreen(pub(crate) bool);
