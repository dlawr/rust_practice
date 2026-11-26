use crate::task_processor::status::Status;
use crate::task_processor::task::Task;
use std::fmt;

pub(crate) fn parse_task(input: &str) -> Result<Task, ParseTaskError> {
    if let Some((str_id, desc)) = input.split_once(":") {
        if desc.len() == 0 {
            return Err(ParseTaskError::EmptyDescription);
        }
        let parsed_id = str_id.parse::<u32>()?;
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

pub(crate) fn parse_all_tasks(inputs: &[&str]) -> Result<Vec<Task>, ParseTaskError> {
    let mut parsed = Vec::new();
    for input in inputs {
        // match parse_task(input) {
        //     Ok(parsed_task) => parsed.push(parsed_task),
        //     Err(error) => return Err(error),
        // }
        parsed.push(parse_task(input)?);
    }
    Ok(parsed)
}

pub(crate) fn find_task(tasks: &[Task], id: u32) -> Option<&Task> {
    tasks.iter().find(|task| task.id == id)
}

pub(crate) fn parse_and_find_task(
    inputs: &[&str],
    id: u32,
) -> Result<Option<Task>, ParseTaskError> {
    let tasks = parse_all_tasks(inputs)?;
    // match find_task(&tasks, id) {
    //     Some(task) => Ok(Some(task.clone())),
    //     _ => Ok(None),
    // }
    Ok(find_task(&tasks, id).cloned())
}

#[derive(Debug)]
pub(crate) enum ParseTaskError {
    MissingSeparator,
    ZeroId,
    EmptyDescription,
    InvalidIdFormat(std::num::ParseIntError),
}

impl fmt::Display for ParseTaskError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            ParseTaskError::EmptyDescription => write!(f, "Empty description"),
            ParseTaskError::MissingSeparator => write!(f, "Missing separator"),
            ParseTaskError::ZeroId => write!(f, "Zero ID"),
            ParseTaskError::InvalidIdFormat(message) => write!(f, "Invalid ID: {}", message),
        }
    }
}

impl From<std::num::ParseIntError> for ParseTaskError {
    fn from(error: std::num::ParseIntError) -> Self {
        Self::InvalidIdFormat(error)
    }
}

impl std::error::Error for ParseTaskError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ParseTaskError::InvalidIdFormat(err) => Some(err),
            _ => None,
        }
    }
}
