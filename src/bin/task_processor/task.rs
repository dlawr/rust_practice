use crate::task_processor::status::Status;

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
