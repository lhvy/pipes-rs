use crate::direction::Direction;
use crossterm::terminal;

pub struct Position {
    pub x: u16,
    pub y: u16,
}

impl Position {
    pub fn can_move_in(&mut self, dir: Direction) -> crossterm::Result<bool> {
        match dir {
            Direction::Up => {
                if self.y == 0 {
                    return Ok(false);
                }
                self.y -= 1;
            }
            Direction::Down => self.y += 1,
            Direction::Left => {
                if self.x == 0 {
                    return Ok(false);
                }
                self.x -= 1;
            }
            Direction::Right => self.x += 1,
        }
        if self.in_screen_bounds()? {
            Ok(true)
        } else {
            Ok(false)
        }
    }

    fn in_screen_bounds(&self) -> crossterm::Result<bool> {
        let (columns, rows) = terminal::size()?;
        Ok(self.x < columns && self.y < rows)
    }
}
