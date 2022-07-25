#[derive(Clone, Copy, PartialEq)]
pub(crate) enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub(crate) fn maybe_turn(&mut self, turn_chance: f32) {
        let will_turn = rng::gen_bool(turn_chance);

        if will_turn {
            let random_turn_dir = TurnDirection::gen();
            self.turn(random_turn_dir);
        }
    }

    fn turn(&mut self, turn_dir: TurnDirection) {
        match turn_dir {
            TurnDirection::Left => self.turn_left(),
            TurnDirection::Right => self.turn_right(),
        }
    }

    fn turn_left(&mut self) {
        *self = match self {
            Self::Up => Self::Left,
            Self::Down => Self::Right,
            Self::Left => Self::Up,
            Self::Right => Self::Down,
        };
    }

    fn turn_right(&mut self) {
        *self = match self {
            Self::Up => Self::Right,
            Self::Down => Self::Left,
            Self::Left => Self::Down,
            Self::Right => Self::Up,
        };
    }
}

enum TurnDirection {
    Left,
    Right,
}

impl TurnDirection {
    fn gen() -> Self {
        if rng::gen_bool(0.5) {
            Self::Left
        } else {
            Self::Right
        }
    }
}

impl Direction {
    pub(crate) fn gen() -> Self {
        match rng::gen_range(0..4) {
            0 => Direction::Up,
            1 => Direction::Down,
            2 => Direction::Left,
            3 => Direction::Right,
            _ => unreachable!(),
        }
    }
}
