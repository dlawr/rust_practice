use std::fmt::Debug;

use crate::task_processor::task::Summary;

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

impl<T: Summary> TaskList<T> {
    pub(crate) fn print_summaries(&self) {
        for task in self.items.iter() {
            println!("{}", task.summary());
        }
    }
    pub(crate) fn summaries(&self) -> Vec<String> {
        self.items.iter().map(|task| task.summary()).collect()
    }
}

impl<T: Summary + Debug> TaskList<T> {
    pub(crate) fn print_debug_summaries(&self) {
        for task in self.items.iter() {
            println!("{} {:?}", task.summary(), task);
        }
    }
}

pub(crate) fn print_items<T: Summary>(items: &[T]) {
    for item in items {
        println!("{}", item.summary());
    }
}
