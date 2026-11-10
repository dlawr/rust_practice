use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub(crate) enum Status {
    Pending,
    InProgress,
    Complete,
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Status::Complete => write!(f, "Complete"),
            Status::InProgress => write!(f, "In Progress"),
            Status::Pending => write!(f, "Pending"),
        }
    }
}
