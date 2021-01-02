use crate::direction::Direction;
use crossterm::terminal;

pub(crate) struct Position {
    pub(crate) x: u16,
    pub(crate) y: u16,
}

impl Position {
    pub(crate) fn move_in(&mut self, dir: Direction) -> Option<()> {
        match dir {
            Direction::Up => self.y = self.y.checked_sub(1)?,
            Direction::Down => self.y = self.y.checked_add(1)?,
            Direction::Left => self.x = self.x.checked_sub(1)?,
            Direction::Right => self.x = self.x.checked_add(1)?,
        }
        // TODO: Error handling.
        if self.in_screen_bounds().unwrap() {
            Some(())
        } else {
            None
        }
    }

    fn in_screen_bounds(&self) -> crossterm::Result<bool> {
        let (columns, rows) = terminal::size()?;
        Ok(self.x < columns && self.y < rows)
    }
}