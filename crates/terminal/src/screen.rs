pub(crate) struct Screen {
    cells: Vec<Cell>,
    cursor: (usize, usize),
    width: usize,
    height: usize,
    num_covered: usize,
}

impl Screen {
    pub(crate) fn new(width: usize, height: usize) -> Self {
        Self {
            cells: vec![Cell { is_covered: false }; width * height],
            cursor: (0, 0),
            width,
            height,
            num_covered: 0,
        }
    }

    pub(crate) fn resize(&mut self, width: usize, height: usize) {
        self.cells
            .resize(width * height, Cell { is_covered: false });
        self.cursor = (0, 0);
        self.width = width;
        self.height = height;
        self.clear();
    }

    pub(crate) fn move_cursor_to(&mut self, x: usize, y: usize) {
        assert!(x < self.width);
        assert!(y < self.height);

        self.cursor = (x, y);
    }

    pub(crate) fn print(&mut self) {
        let current_cell = self.current_cell();
        if !current_cell.is_covered {
            current_cell.is_covered = true;
            self.num_covered += 1;
        }
    }

    pub(crate) fn clear(&mut self) {
        for cell in &mut self.cells {
            cell.is_covered = false;
        }
        self.num_covered = 0;
    }

    pub(crate) fn portion_covered(&self) -> f32 {
        debug_assert_eq!(
            self.num_covered,
            self.cells.iter().filter(|c| c.is_covered).count()
        );
        self.num_covered as f32 / self.cells.len() as f32
    }

    fn current_cell(&mut self) -> &mut Cell {
        &mut self.cells[self.cursor.1 * self.width + self.cursor.0]
    }
}

#[derive(Clone, Copy)]
struct Cell {
    is_covered: bool,
}
