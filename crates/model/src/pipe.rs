mod color;
mod kind;

pub use color::{ColorMode, Palette};
pub use kind::{Kind, KindSet};

use self::color::Color;
use crate::direction::Direction;
use crate::position::{InScreenBounds, Position};

pub struct Pipe {
    current_direction: Direction,
    previous_direction: Direction,
    pub position: Position,
    pub color: Option<Color>,
    kind: Kind,
}

impl Pipe {
    pub fn new(size: (u16, u16), color_mode: ColorMode, palette: Palette, kind: Kind) -> Self {
        let color = color::gen_random_color(color_mode, palette);
        let (direction, position) = gen_random_direction_and_position(size);

        Self {
            current_direction: direction,
            previous_direction: direction,
            position,
            color,
            kind,
        }
    }

    pub fn dup(&self, size: (u16, u16)) -> Self {
        let (direction, position) = gen_random_direction_and_position(size);

        Self {
            current_direction: direction,
            previous_direction: direction,
            position,
            color: self.color,
            kind: self.kind,
        }
    }

    pub fn tick(&mut self, size: (u16, u16), turn_chance: f32, hue_shift: u8) -> InScreenBounds {
        let InScreenBounds(in_screen_bounds) = self.position.move_in(self.current_direction, size);

        if let Some(color) = &mut self.color {
            color.update(hue_shift.into());
        }

        if !in_screen_bounds {
            return InScreenBounds(false);
        }

        self.previous_direction = self.current_direction;
        self.current_direction = self.current_direction.maybe_turn(turn_chance);

        InScreenBounds(true)
    }

    pub fn to_char(&self) -> char {
        match (self.previous_direction, self.current_direction) {
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
}

fn gen_random_direction_and_position((columns, rows): (u16, u16)) -> (Direction, Position) {
    let direction = Direction::gen();
    let position = match direction {
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

    (direction, position)
}
