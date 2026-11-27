use crate::task_processor::parsing::{ParseTaskError, parse_task};
use crate::task_processor::status::Status;
use std::str::FromStr;

#[derive(Debug, PartialEq, Clone)]
pub(crate) struct Task {
    pub(crate) id: u32,
    pub(crate) description: String,
    pub(crate) status: Status,
}

pub(crate) trait Summary {
    fn summary(&self) -> String;
}

impl Summary for Task {
    fn summary(&self) -> String {
        format!("Task #{}: {} [{}]", self.id, self.description, self.status)
    }
}

impl FromStr for Task {
    type Err = ParseTaskError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse_task(s)
    }
}

impl std::fmt::Display for Task {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {} [{}]", self.id, self.description, self.status)
    }
}
