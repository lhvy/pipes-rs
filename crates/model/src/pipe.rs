mod color;
mod history_keeper;
mod kind;

pub use color::{ColorMode, Palette};
use history_keeper::HistoryKeeper;
use kind::Kind;
pub use kind::{PresetKind, PresetKindSet};

use crate::direction::Direction;
use crate::position::InScreenBounds;
use crate::position::Position;
use rng::Rng;

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
