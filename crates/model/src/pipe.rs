mod color;
mod history_keeper;

pub use color::{ColorMode, Palette};
use history_keeper::HistoryKeeper;

use crate::direction::Direction;
use crate::position::InScreenBounds;
use crate::position::Position;
use rng::Rng;
use std::{collections::HashSet, str::FromStr};

pub struct Pipe {
    dir: HistoryKeeper<Direction>,
    pub pos: Position,
    pub color: Option<terminal::Color>,
    kind: Kind,
}

impl Pipe {
    pub fn new(
        size: (u16, u16),
        rng: &mut Rng,
        color_mode: ColorMode,
        palette: Palette,
        preset_kind: PresetKind,
    ) -> Self {
        let color = color::gen_random_color(rng, color_mode, palette);
        Self::new_raw(size, rng, color, preset_kind.kind())
    }

    fn new_raw(
        size: (u16, u16),
        rng: &mut Rng,
        color: Option<terminal::Color>,
        kind: Kind,
    ) -> Self {
        let (dir, pos) = Self::gen_rand_dir_and_pos(size, rng);

        Self {
            dir: HistoryKeeper::new(dir),
            pos,
            color,
            kind,
        }
    }

    fn gen_rand_dir_and_pos((columns, rows): (u16, u16), rng: &mut Rng) -> (Direction, Position) {
        let dir = Direction::gen(rng);
        let pos = match dir {
            Direction::Up => Position {
                x: rng.gen_range_16(0..columns),
                y: rows - 1,
            },
            Direction::Down => Position {
                x: rng.gen_range_16(0..columns),
                y: 0,
            },
            Direction::Left => Position {
                x: columns - 1,
                y: rng.gen_range_16(0..rows),
            },
            Direction::Right => Position {
                x: 0,
                y: rng.gen_range_16(0..rows),
            },
        };

        (dir, pos)
    }

    pub fn dup(&self, size: (u16, u16), rng: &mut Rng) -> Self {
        Self::new_raw(size, rng, self.color, self.kind)
    }

    pub fn tick(&mut self, size: (u16, u16), rng: &mut Rng, turn_chance: f32) -> InScreenBounds {
        let InScreenBounds(in_screen_bounds) = self.pos.move_in(self.dir.current(), size);

        if !in_screen_bounds {
            return InScreenBounds(false);
        }

        self.dir.update(|dir| dir.maybe_turn(rng, turn_chance));

        InScreenBounds(true)
    }

    pub fn to_char(&self) -> char {
        let dir = self.dir.current();
        let prev_dir = self.dir.previous().unwrap_or(dir);

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
    Knobby,
    Emoji,
    Outline,
    Dots,
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
    width: KindWidth,
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

#[derive(Clone, Copy)]
enum KindWidth {
    Auto,
    Custom(usize),
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
        width: KindWidth::Auto,
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
        width: KindWidth::Auto,
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
        width: KindWidth::Auto,
    };

    const KNOBBY: Kind = Kind {
        up: '╽',
        down: '╿',
        left: '╼',
        right: '╾',
        top_left: '┎',
        top_right: '┒',
        bottom_left: '┖',
        bottom_right: '┚',
        width: KindWidth::Auto,
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
        width: KindWidth::Auto,
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
        width: KindWidth::Auto,
    };

    const DOTS: Kind = Kind {
        up: '•',
        down: '•',
        left: '•',
        right: '•',
        top_left: '•',
        top_right: '•',
        bottom_left: '•',
        bottom_right: '•',
        width: KindWidth::Custom(2),
    };

    fn kind(&self) -> Kind {
        match self {
            Self::Heavy => Self::HEAVY,
            Self::Light => Self::LIGHT,
            Self::Curved => Self::CURVED,
            Self::Knobby => Self::KNOBBY,
            Self::Emoji => Self::EMOJI,
            Self::Outline => Self::OUTLINE,
            Self::Dots => Self::DOTS,
        }
    }
}

impl FromStr for PresetKind {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "heavy" => Self::Heavy,
            "light" => Self::Light,
            "curved" => Self::Curved,
            "knobby" => Self::Knobby,
            "emoji" => Self::Emoji,
            "outline" => Self::Outline,
            "dots" => Self::Dots,
            _ => anyhow::bail!(
                r#"unknown pipe kind (expected “heavy”, “light”, “curved”, “knobby”, “emoji”, “outline” or “dots”)"#,
            ),
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
    pub fn chars(&self) -> impl Iterator<Item = char> + '_ {
        self.kinds().flat_map(|kind| kind.chars())
    }

    pub fn custom_widths(&self) -> impl Iterator<Item = usize> + '_ {
        self.kinds().filter_map(|kind| match kind.width {
            KindWidth::Custom(n) => Some(n),
            KindWidth::Auto => None,
        })
    }

    fn kinds(&self) -> impl Iterator<Item = Kind> + '_ {
        self.0.iter().map(|preset_kind| preset_kind.kind())
    }
}
