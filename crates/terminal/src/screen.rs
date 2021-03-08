use crate::Grapheme;

pub(crate) struct Screen<'cell_text> {
    text: Vec<Cell<'cell_text>>,
    cursor: (usize, usize),
    width: usize,
    height: usize,
}

impl<'cell_text> Screen<'cell_text> {
    pub(crate) fn new(width: usize, height: usize) -> Self {
        Self {
            text: vec![Cell::empty(); width * height],
            cursor: (0, 0),
            width,
            height,
        }
    }

    pub(crate) fn move_cursor_to(&mut self, x: usize, y: usize) {
        assert!(x < self.width);
        assert!(y < self.height);

        self.cursor = (x, y);
    }

    pub(crate) fn print(&mut self, grapheme: Grapheme<'cell_text>) {
        *self.current_cell() = Cell {
            grapheme: Some(grapheme),
        };
    }

    pub(crate) fn clear(&mut self) {
        self.text = vec![Cell::empty(); self.width * self.height];
    }

    pub(crate) fn portion_covered(&self) -> f32 {
        let num_covered = self.text.iter().filter(|c| c.is_covered()).count();
        let total = self.text.len();

        num_covered as f32 / total as f32
    }

    fn current_cell(&mut self) -> &mut Cell<'cell_text> {
        &mut self.text[self.cursor.1 * self.width + self.cursor.0]
    }
}

#[derive(Clone, Copy)]
struct Cell<'a> {
    grapheme: Option<Grapheme<'a>>,
}

impl Cell<'_> {
    fn empty() -> Self {
        Self { grapheme: None }
    }

    fn is_covered(self) -> bool {
        self.grapheme.is_some()
    }
}
