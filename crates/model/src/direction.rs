use rng::Rng;

#[derive(Clone, Copy, PartialEq)]
pub(crate) enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub(crate) fn maybe_turn(&mut self, rng: &mut Rng, turn_chance: f32) {
        let will_turn = rng.gen_bool(turn_chance);

        if will_turn {
            *self = self.turn(TurnDirection::gen(rng));
        }
    }

    fn turn(&self, turn_dir: TurnDirection) -> Self {
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
    fn gen(rng: &mut Rng) -> Self {
        if rng.gen_bool(0.5) {
            Self::Left
        } else {
            Self::Right
        }
    }
}

impl Direction {
    pub(crate) fn gen(rng: &mut Rng) -> Self {
        match rng.gen_range(0..4) {
            0 => Direction::Up,
            1 => Direction::Down,
            2 => Direction::Left,
            3 => Direction::Right,
            _ => unreachable!(),
        }
    }
}
