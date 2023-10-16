#[derive(Clone, Copy, PartialEq)]
pub(crate) enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub(crate) fn maybe_turn(self, turn_chance: f32) -> Direction {
        if !rng::gen_bool(turn_chance) {
            return self;
        }

        if rng::gen_bool(0.5) {
            // turn left
            match self {
                Direction::Up => Direction::Left,
                Direction::Down => Direction::Right,
                Direction::Left => Direction::Up,
                Direction::Right => Direction::Down,
            }
        } else {
            // turn right
            match self {
                Direction::Up => Direction::Right,
                Direction::Down => Direction::Left,
                Direction::Left => Direction::Down,
                Direction::Right => Direction::Up,
            }
        }
    }
}
