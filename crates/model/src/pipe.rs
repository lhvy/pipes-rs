mod color;
mod history_keeper;
mod kind;

pub use color::{ColorMode, Palette};
use history_keeper::HistoryKeeper;
pub use kind::{Kind, KindSet};

use crate::direction::Direction;
use crate::position::{InScreenBounds, Position};

pub struct Pipe {
    dir: HistoryKeeper<Direction>,
    pub pos: Position,
    pub color: Option<terminal::Color>,
    kind: Kind,
}

impl Pipe {
    pub fn new(size: (u16, u16), color_mode: ColorMode, palette: Palette, kind: Kind) -> Self {
        let color = color::gen_random_color(color_mode, palette);
        let (dir, pos) = Self::gen_rand_dir_and_pos(size);

        Self {
            dir: HistoryKeeper::new(dir),
            pos,
            color,
            kind,
        }
    }

    pub fn dup(&self, size: (u16, u16)) -> Self {
        let (dir, pos) = Self::gen_rand_dir_and_pos(size);

        Self {
            dir: HistoryKeeper::new(dir),
            pos,
            color: self.color,
            kind: self.kind,
        }
    }

    pub fn tick(&mut self, size: (u16, u16), turn_chance: f32) -> InScreenBounds {
        let InScreenBounds(in_screen_bounds) = self.pos.move_in(self.dir.current(), size);

        if !in_screen_bounds {
            return InScreenBounds(false);
        }

        self.dir.update(|dir| dir.maybe_turn(turn_chance));

        InScreenBounds(true)
    }

    pub fn to_char(&self) -> char {
        let dir = self.dir.current();
        let prev_dir = self.dir.previous().unwrap_or(dir);

        match (prev_dir, dir) {
            (Direction::Up, Direction::Left) | (Direction::Right, Direction::Down) => {
                self.kind.top_right()
            }
            (Direction::Up, Direction::Right) | (Direction::Left, Direction::Down) => {
                self.kind.top_left()
            }
            (Direction::Down, Direction::Left) | (Direction::Right, Direction::Up) => {
                self.kind.bottom_right()
            }
            (Direction::Down, Direction::Right) | (Direction::Left, Direction::Up) => {
                self.kind.bottom_left()
            }
            (Direction::Up, Direction::Up) => self.kind.up(),
            (Direction::Down, Direction::Down) => self.kind.down(),
            (Direction::Left, Direction::Left) => self.kind.left(),
            (Direction::Right, Direction::Right) => self.kind.right(),
            _ => unreachable!(),
        }
    }

    fn gen_rand_dir_and_pos((columns, rows): (u16, u16)) -> (Direction, Position) {
        let dir = Direction::gen();
        let pos = match dir {
            Direction::Up => Position {
                x: rng::gen_range_16(0..columns),
                y: rows - 1,
            },
            Direction::Down => Position {
                x: rng::gen_range_16(0..columns),
                y: 0,
            },
            Direction::Left => Position {
                x: columns - 1,
                y: rng::gen_range_16(0..rows),
            },
            Direction::Right => Position {
                x: 0,
                y: rng::gen_range_16(0..rows),
            },
        };

        (dir, pos)
    }
}
