use crate::direction::Direction;
use crate::position::Position;
use crossterm::{style, terminal};
use rand::Rng;

pub(crate) struct Pipe {
    pub(crate) dir: Direction,
    pub(crate) pos: Position,
    pub(crate) color: style::Color,
    prev_dir: Direction,
    just_turned: bool,
}

impl Pipe {
    pub(crate) fn new() -> crossterm::Result<Self> {
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
        Ok(Self {
            dir,
            pos,
            color: gen_random_color(),
            prev_dir: dir,
            just_turned: false,
        })
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
                (Direction::Up, Direction::Left) | (Direction::Right, Direction::Down) => '┓',
                (Direction::Up, Direction::Right) | (Direction::Left, Direction::Down) => '┏',
                (Direction::Down, Direction::Left) | (Direction::Right, Direction::Up) => '┛',
                (Direction::Down, Direction::Right) | (Direction::Left, Direction::Up) => '┗',
                _ => unreachable!(),
            }
        } else {
            self.dir.to_char()
        }
    }
}

fn gen_random_color() -> style::Color {
    let hue = rand::thread_rng().gen_range(0.0..=360.0);
    let lch = color_space::Lch {
        l: 75.0,
        c: 75.0,
        h: hue,
    };
    let color_space::Rgb { r, g, b } = color_space::Rgb::from(lch);
    style::Color::Rgb {
        r: r as u8,
        g: g as u8,
        b: b as u8,
    }
}

#[derive(PartialEq)]
pub(crate) struct IsOffScreen(pub(crate) bool);
