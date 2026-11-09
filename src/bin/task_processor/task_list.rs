pub(crate) struct TaskList<T> {
    items: Vec<T>,
}

impl<T> TaskList<T> {
    pub(crate) fn new() -> Self {
        Self { items: Vec::new() }
    }
    pub(crate) fn add(&mut self, item: T) {
        self.items.push(item);
    }
    pub(crate) fn len(&self) -> usize {
        self.items.len()
    }
    pub(crate) fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
