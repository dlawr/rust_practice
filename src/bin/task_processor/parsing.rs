use crate::task_processor::status::Status;
use crate::task_processor::task::Task;
use std::fmt;

pub(crate) fn parse_task(input: &str) -> Result<Task, ParseTaskError> {
    if let Some((str_id, desc)) = input.split_once(":") {
        if desc.len() == 0 {
            return Err(ParseTaskError::EmptyDescription);
        }
        let parsed_id = str_id
            .parse::<u32>()
            .map_err(|_e| ParseTaskError::InvalidId)?;
        if parsed_id <= 0 {
            return Err(ParseTaskError::ZeroId);
        }
        Ok(Task {
            id: parsed_id,
            description: desc.to_string(),
            status: Status::Pending,
        })
    } else {
        Err(ParseTaskError::MissingSeparator)
    }
}

pub(crate) fn parse_tasks(tasks: &[&str]) -> (Vec<Task>, Vec<ParseTaskError>) {
    let mut parsed = Vec::new();
    let mut errors = Vec::new();
    for task in tasks {
        match parse_task(task) {
            Ok(parsed_task) => parsed.push(parsed_task),
            Err(error) => errors.push(error),
        }
    }
    (parsed, errors)
}

#[derive(Debug, PartialEq)]
pub(crate) enum ParseTaskError {
    MissingSeparator,
    InvalidId,
    ZeroId,
    EmptyDescription,
}

impl fmt::Display for ParseTaskError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            ParseTaskError::EmptyDescription => write!(f, "Empty description"),
            ParseTaskError::InvalidId => write!(f, "Invalid ID"),
            ParseTaskError::MissingSeparator => write!(f, "Missing separator"),
            ParseTaskError::ZeroId => write!(f, "Zero ID"),
        }
    }
}
