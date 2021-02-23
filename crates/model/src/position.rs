use crate::direction::Direction;
use terminal::Terminal;

pub struct Position {
    pub x: u16,
    pub y: u16,
}

impl Position {
    pub(crate) fn move_in(&mut self, dir: Direction, terminal: &mut Terminal) -> InScreenBounds {
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

        InScreenBounds(self.in_screen_bounds(terminal))
    }

    fn in_screen_bounds(&self, terminal: &mut Terminal) -> bool {
        let (columns, rows) = terminal.size();
        self.x < columns && self.y < rows
    }
}

#[derive(PartialEq)]
pub struct InScreenBounds(pub bool);
