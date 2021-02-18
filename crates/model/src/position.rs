use crate::direction::Direction;
use terminal::Terminal;

pub struct Position {
    pub x: u16,
    pub y: u16,
}

impl Position {
    pub(crate) fn move_in(
        &mut self,
        dir: Direction,
        terminal: &mut Terminal,
    ) -> anyhow::Result<InScreenBounds> {
        match dir {
            Direction::Up => {
                if self.y == 0 {
                    return Ok(InScreenBounds(false));
                }
                self.y -= 1;
            }
            Direction::Down => self.y += 1,
            Direction::Left => {
                if self.x == 0 {
                    return Ok(InScreenBounds(false));
                }
                self.x -= 1;
            }
            Direction::Right => self.x += 1,
        }

        self.in_screen_bounds(terminal).map(InScreenBounds)
    }

    fn in_screen_bounds(&self, terminal: &mut Terminal) -> anyhow::Result<bool> {
        let (columns, rows) = terminal.size()?;
        Ok(self.x < columns && self.y < rows)
    }
}

#[derive(PartialEq)]
pub struct InScreenBounds(pub bool);
