use crate::direction::Direction;
use crate::position::Position;
use crate::{config::ColorMode, position::InScreenBounds};
use rand::Rng;
use std::{collections::HashSet, str::FromStr};
use terminal::Terminal;

pub struct Pipe {
    pub dirs: Vec<Direction>,
    pub pos: Position,
    pub color: Option<terminal::Color>,
    kind: Kind,
}

impl Pipe {
    pub fn new(terminal: &mut Terminal, color_mode: ColorMode, preset_kind: PresetKind) -> Self {
        Self::new_raw(terminal, gen_random_color(color_mode), preset_kind.kind())
    }

    fn new_raw(terminal: &mut Terminal, color: Option<terminal::Color>, kind: Kind) -> Self {
        let (dir, pos) = Self::gen_rand_dir_and_pos(terminal);

        Self {
            dirs: vec![dir],
            pos,
            color,
            kind,
        }
    }

    fn gen_rand_dir_and_pos(terminal: &mut Terminal) -> (Direction, Position) {
        let (columns, rows) = terminal.size();
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

        (dir, pos)
    }

    pub fn dup(&self, terminal: &mut Terminal) -> Self {
        Self::new_raw(terminal, self.color, self.kind)
    }

    pub fn tick(&mut self, terminal: &mut Terminal) -> InScreenBounds {
        let InScreenBounds(in_screen_bounds) =
            self.pos.move_in(self.dirs[self.dirs.len() - 1], terminal);

        if !in_screen_bounds {
            return InScreenBounds(false);
        }

        self.dirs.push(*self.dirs.last().unwrap());
        self.dirs.last_mut().unwrap().maybe_turn();

        InScreenBounds(true)
    }

    pub fn to_char(&self) -> char {
        let dir = self.dirs[self.dirs.len() - 1];
        let prev_dir = self
            .dirs
            .len()
            .checked_sub(2)
            .map_or(dir, |idx| self.dirs[idx]);

        match (prev_dir, dir) {
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
            (Direction::Up, Direction::Up) => self.kind.up,
            (Direction::Down, Direction::Down) => self.kind.down,
            (Direction::Left, Direction::Left) => self.kind.left,
            (Direction::Right, Direction::Right) => self.kind.right,
            _ => unreachable!(),
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize, Eq, PartialEq, Hash, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum PresetKind {
    Heavy,
    Light,
    Curved,
    Emoji,
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

impl Kind {
    fn chars(&self) -> Vec<char> {
        vec![
            self.up,
            self.down,
            self.left,
            self.right,
            self.top_left,
            self.top_right,
            self.bottom_left,
            self.bottom_right,
        ]
    }
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
            Self::Emoji => Self::EMOJI,
            Self::Outline => Self::OUTLINE,
        }
    }
}

fn gen_random_color(color_mode: ColorMode) -> Option<terminal::Color> {
    match color_mode {
        ColorMode::Ansi => {
            let num = rand::thread_rng().gen_range(0..=11);
            Some(match num {
                0 => terminal::Color::Red,
                1 => terminal::Color::DarkRed,
                2 => terminal::Color::Green,
                3 => terminal::Color::DarkGreen,
                4 => terminal::Color::Yellow,
                5 => terminal::Color::DarkYellow,
                6 => terminal::Color::Blue,
                7 => terminal::Color::DarkBlue,
                8 => terminal::Color::Magenta,
                9 => terminal::Color::DarkMagenta,
                10 => terminal::Color::Cyan,
                11 => terminal::Color::DarkCyan,
                _ => unreachable!(),
            })
        }
        ColorMode::Rgb => {
            let hue = rand::thread_rng().gen_range(0.0..=360.0);
            let oklch = tincture::Oklch {
                l: 0.75,
                c: 0.125,
                h: tincture::Hue::from_degrees(hue).unwrap(),
            };
            let oklab = tincture::Oklab::from(oklch);
            let lrgb: tincture::LinearRgb = tincture::convert(oklab);
            let tincture::Srgb { r, g, b } = tincture::Srgb::from(lrgb);
            Some(terminal::Color::Rgb {
                r: (r * 255.0) as u8,
                g: (g * 255.0) as u8,
                b: (b * 255.0) as u8,
            })
        }
        ColorMode::None => None,
    }
}

impl FromStr for PresetKind {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "heavy" => Self::Heavy,
            "light" => Self::Light,
            "curved" => Self::Curved,
            "emoji" => Self::Emoji,
            "outline" => Self::Outline,
            _ => anyhow::bail!(r#"unknown pipe kind"#),
        })
    }
}

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct PresetKindSet(pub HashSet<PresetKind>);

impl FromStr for PresetKindSet {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut set = HashSet::new();
        for preset_kind in s.split(',') {
            set.insert(PresetKind::from_str(preset_kind)?);
        }
        Ok(Self(set))
    }
}

impl PresetKindSet {
    pub fn chars(&self) -> Vec<char> {
        self.0
            .iter()
            .map(|preset_kind| preset_kind.kind())
            .flat_map(|kind| kind.chars())
            .collect()
    }
}
