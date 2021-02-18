use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

#[derive(Clone, Copy, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn will_turn() -> bool {
        rand::thread_rng().gen_bool(0.15)
    }

    pub fn maybe_turn(&mut self) {
        if Self::will_turn() {
            *self = self.turn(TurnDirection::gen());
        }
    }

    fn turn(&mut self, turn_dir: TurnDirection) -> Self {
        match turn_dir {
            TurnDirection::Left => match self {
                Self::Up => Self::Left,
                Self::Down => Self::Right,
                Self::Left => Self::Up,
                Self::Right => Self::Down,
            },
            TurnDirection::Right => match self {
                Self::Up => Self::Right,
                Self::Down => Self::Left,
                Self::Left => Self::Down,
                Self::Right => Self::Up,
            },
        }
    }
}

enum TurnDirection {
    Left,
    Right,
}

impl TurnDirection {
    fn gen() -> Self {
        if rand::thread_rng().gen_bool(0.5) {
            Self::Left
        } else {
            Self::Right
        }
    }
}

impl Distribution<Direction> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Direction {
        match rng.gen_range(0..=3) {
            0 => Direction::Up,
            1 => Direction::Down,
            2 => Direction::Left,
            3 => Direction::Right,
            _ => unreachable!(),
        }
    }
}
