use crate::direction::Direction;

pub struct Position {
    pub x: u16,
    pub y: u16,
}

impl Position {
    pub(crate) fn move_in(&mut self, dir: Direction, size: (u16, u16)) -> InScreenBounds {
        match dir {
            Direction::Up => {
                if self.y == 0 {
                    return InScreenBounds(false);
                }
                self.y -= 1;
            }
            Direction::Down => self.y += 1,
            Direction::Left => {
                if self.x == 0 {
                    return InScreenBounds(false);
                }
                self.x -= 1;
            }
            Direction::Right => self.x += 1,
        }

        InScreenBounds(self.in_screen_bounds(size))
    }

    fn in_screen_bounds(&self, (columns, rows): (u16, u16)) -> bool {
        self.x < columns && self.y < rows
    }
}

#[derive(PartialEq)]
pub struct InScreenBounds(pub bool);
