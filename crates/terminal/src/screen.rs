pub(crate) struct Screen {
    text: Vec<Cell>,
    cursor: (usize, usize),
    width: usize,
    height: usize,
}

impl Screen {
    pub(crate) fn new(width: usize, height: usize) -> Self {
        Self {
            text: vec![Cell(None); width * height],
            cursor: (0, 0),
            width,
            height,
        }
    }

    pub(crate) fn move_cursor_to(&mut self, x: usize, y: usize) {
        self.cursor = (x, y);
    }

    pub(crate) fn print(&mut self, c: char) {
        *self.current_cell() = Cell(Some(c));
    }

    pub(crate) fn clear(&mut self) {
        self.text = vec![Cell(None); self.width * self.height];
    }

    fn current_cell(&mut self) -> &mut Cell {
        &mut self.text[self.cursor.1 * self.width + self.cursor.0]
    }
}

#[derive(Clone)]
struct Cell(Option<char>);
