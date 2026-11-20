use crate::task_processor::status::Status;
use crate::task_processor::task::Task;

pub(crate) fn parse_task(input: &str) -> Result<Task, String> {
    if let Some((str_id, desc)) = input.split_once(":") {
        if desc.len() == 0 {
            return Err("Error: missing description".to_string());
        }
        let parsed_id = str_id
            .parse::<u32>()
            .map_err(|e| format!("Invalid ID: {}", e))?;
        if parsed_id <= 0 {
            return Err("Error: invalid id".to_string());
        }
        Ok(Task {
            id: parsed_id,
            description: desc.to_string(),
            status: Status::Pending,
        })
    } else {
        Err("Error: missing data".to_string())
    }
}

pub(crate) fn parse_tasks(tasks: &[&str]) -> (Vec<Task>, Vec<String>) {
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
