pub(super) struct HistoryKeeper<T: Copy> {
    previous: Option<T>,
    current: T,
}

impl<T: Copy> HistoryKeeper<T> {
    pub(super) fn new(value: T) -> Self {
        Self {
            previous: None,
            current: value,
        }
    }

    pub(super) fn current(&self) -> T {
        self.current
    }

    pub(super) fn previous(&self) -> Option<T> {
        self.previous
    }

    pub(super) fn update(&mut self, mut f: impl FnMut(&mut T)) {
        self.previous = Some(self.current);
        f(&mut self.current);
    }
}
