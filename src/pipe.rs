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

    pub(crate) fn tick(&mut self) -> Option<()> {
        self.pos.move_in(self.dir)?;
        self.prev_dir = self.dir;
        self.just_turned = self.dir.maybe_turn();
        Some(())
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
    let num = rand::thread_rng().gen_range(0..=5);
    match num {
        0 => style::Color::Red,
        1 => style::Color::Green,
        2 => style::Color::Yellow,
        3 => style::Color::Blue,
        4 => style::Color::Magenta,
        5 => style::Color::Cyan,
        _ => unreachable!(),
    }
}
