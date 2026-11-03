use std::os::unix::process::parent_id;

struct Task {
    id: u32,
    description: String,
}

fn parse_task(input: &str) -> Result<Task, String> {
    if let Some((str_id, desc)) = input.split_once(":") {
        let parsed_id = str_id
            .parse::<u32>()
            .map_err(|e| format!("Invalid ID: {}", e))?;
        Ok(Task {
            id: parsed_id,
            description: desc.to_string(),
        })
    } else {
        Err("Error: missing data".to_string())
    }
}

// fn parse_tasks(tasks: &[&str]) -> (Vec<Task>, Vec<String>) {}

fn main() {
    let tasks = vec![
        "1:Buy groceries",
        "2:Finish Rust exercise",
        "invalid task",
        "3:Read a book",
        "4:Review http://example.com",
        "5:Meeting at 10:30",
    ];
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_valid_task() {}
}
