use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

#[derive(Clone, Copy)]
pub(crate) enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn will_turn() -> bool {
        rand::thread_rng().gen_bool(0.15)
    }

    pub(crate) fn maybe_turn(&mut self) -> bool {
        if Self::will_turn() {
            let left = rand::thread_rng().gen_bool(0.5);
            *self = if left {
                match self {
                    Self::Up => Self::Left,
                    Self::Down => Self::Right,
                    Self::Left => Self::Up,
                    Self::Right => Self::Down,
                }
            } else {
                match self {
                    Self::Up => Self::Right,
                    Self::Down => Self::Left,
                    Self::Left => Self::Down,
                    Self::Right => Self::Up,
                }
            };
            true
        } else {
            false
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
