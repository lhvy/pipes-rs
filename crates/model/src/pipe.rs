mod color;
pub use color::{ColorMode, Palette};
use terminal::Grapheme;

use crate::direction::Direction;
use crate::position::InScreenBounds;
use crate::position::Position;
use rng::Rng;
use std::{collections::HashSet, str::FromStr};

pub struct Pipe {
    dirs: Vec<Direction>,
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
            dirs: vec![dir],
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
        let InScreenBounds(in_screen_bounds) =
            self.pos.move_in(self.dirs[self.dirs.len() - 1], size);

        if !in_screen_bounds {
            return InScreenBounds(false);
        }

        self.dirs.push(*self.dirs.last().unwrap());
        self.dirs.last_mut().unwrap().maybe_turn(rng, turn_chance);

        InScreenBounds(true)
    }

    pub fn to_grapheme(&self) -> Grapheme<'static> {
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
    Knobby,
    Emoji,
    Outline,
    Dots,
}

#[derive(Clone, Copy)]
struct Kind {
    up: Grapheme<'static>,
    down: Grapheme<'static>,
    left: Grapheme<'static>,
    right: Grapheme<'static>,
    top_left: Grapheme<'static>,
    top_right: Grapheme<'static>,
    bottom_left: Grapheme<'static>,
    bottom_right: Grapheme<'static>,
}

impl Kind {
    fn graphemes(&self) -> Vec<Grapheme<'static>> {
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

macro_rules! define_preset {
    (
        $name:ident,
        $up:literal,
        $down: literal,
        $left:literal,
        $right:literal,
        $top_left:literal,
        $top_right:literal,
        $bottom_left:literal,
        $bottom_right:literal
    ) => {
        fn $name() -> Kind {
            Kind {
                up: Grapheme::new($up).unwrap(),
                down: Grapheme::new($down).unwrap(),
                left: Grapheme::new($left).unwrap(),
                right: Grapheme::new($right).unwrap(),
                top_left: Grapheme::new($top_left).unwrap(),
                top_right: Grapheme::new($top_right).unwrap(),
                bottom_left: Grapheme::new($bottom_left).unwrap(),
                bottom_right: Grapheme::new($bottom_right).unwrap(),
            }
        }
    };
}

impl PresetKind {
    fn kind(&self) -> Kind {
        match self {
            Self::Heavy => Self::heavy(),
            Self::Light => Self::light(),
            Self::Curved => Self::curved(),
            Self::Knobby => Self::knobby(),
            Self::Emoji => Self::emoji(),
            Self::Outline => Self::outline(),
            Self::Dots => Self::dots(),
        }
    }

    define_preset!(heavy, "┃", "┃", "━", "━", "┏", "┓", "┗", "┛");
    define_preset!(light, "│", "│", "─", "─", "┌", "┐", "└", "┘");
    define_preset!(curved, "│", "│", "─", "─", "╭", "╮", "╰", "╯");
    define_preset!(knobby, "╽", "╿", "╼", "╾", "┎", "┒", "┖", "┚");
    define_preset!(emoji, "👆", "👇", "👈", "👉", "👌", "👌", "👌", "👌");
    define_preset!(outline, "║", "║", "═", "═", "╔", "╗", "╚", "╝");
    define_preset!(dots, "•", "•", "•", "•", "•", "•", "•", "•");
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
    pub fn graphemes(&self) -> impl Iterator<Item = Grapheme<'static>> + '_ {
        self.0
            .iter()
            .map(|preset_kind| preset_kind.kind())
            .flat_map(|kind| kind.graphemes())
    }
}
