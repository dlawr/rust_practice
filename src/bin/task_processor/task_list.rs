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
    pub(crate) fn find_by_summary(&self, query: &str) -> Vec<&T> {
        let mut found = Vec::new();
        for item in &self.items {
            if item
                .summary()
                .to_lowercase()
                .contains(&query.to_lowercase())
            {
                found.push(item);
            }
        }
        return found;
    }
}

impl<T: Summary + Debug> TaskList<T> {
    pub(crate) fn print_debug_summaries(&self) {
        for task in self.items.iter() {
            println!("{} {:?}", task.summary(), task);
        }
    }
}

impl<T: Summary + Clone> TaskList<T> {
    pub(crate) fn find_cloned_by_summary(&self, query: &str) -> Vec<T> {
        let mut found = Vec::new();
        for item in &self.items {
            if item
                .summary()
                .to_lowercase()
                .contains(&query.to_lowercase())
            {
                found.push(item.clone());
            }
        }
        return found;
    }
}

impl<'a, T> IntoIterator for &'a TaskList<T> {
    type Item = &'a T;
    type IntoIter = std::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.items.iter()
    }
}

impl<T> IntoIterator for TaskList<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.items.into_iter()
    }
}

impl<'a, T> IntoIterator for &'a mut TaskList<T> {
    type Item = &'a mut T;
    type IntoIter = std::slice::IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.items.iter_mut()
    }
}

pub(crate) fn print_items<T: Summary>(items: &[T]) {
    for item in items {
        println!("{}", item.summary());
    }
}
