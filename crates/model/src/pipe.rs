use crate::config::ColorMode;
use crate::direction::Direction;
use crate::position::Position;
use rand::Rng;
use std::{collections::HashSet, str::FromStr};
use terminal::Terminal;

pub struct Pipe {
    pub dir: Direction,
    pub pos: Position,
    pub color: Option<terminal::Color>,
    kind: Kind,
    prev_dir: Direction,
    just_turned: bool,
}

impl Pipe {
    pub fn new(
        terminal: &mut Terminal,
        color_mode: ColorMode,
        preset_kind: PresetKind,
    ) -> anyhow::Result<Self> {
        Self::new_raw(terminal, gen_random_color(color_mode), preset_kind.kind())
    }

    fn new_raw(
        terminal: &mut Terminal,
        color: Option<terminal::Color>,
        kind: Kind,
    ) -> anyhow::Result<Self> {
        let (dir, pos) = Self::gen_rand_dir_and_pos(terminal)?;
        Ok(Self {
            dir,
            pos,
            color,
            kind,
            prev_dir: dir,
            just_turned: false,
        })
    }

    fn gen_rand_dir_and_pos(terminal: &mut Terminal) -> anyhow::Result<(Direction, Position)> {
        let (columns, rows) = terminal.size()?;
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

    pub fn dup(&self, terminal: &mut Terminal) -> anyhow::Result<Self> {
        Self::new_raw(terminal, self.color, self.kind)
    }

    pub fn tick(&mut self, terminal: &mut Terminal) -> anyhow::Result<IsOffScreen> {
        if !self.pos.can_move_in(self.dir, terminal)? {
            return Ok(IsOffScreen(true));
        }
        self.prev_dir = self.dir;
        self.just_turned = self.dir.maybe_turn();
        Ok(IsOffScreen(false))
    }

    pub fn to_char(&self) -> char {
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

#[derive(PartialEq)]
pub struct IsOffScreen(pub bool);

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
