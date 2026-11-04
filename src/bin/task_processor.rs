// use std::os::unix::process::parent_id;

#[derive(Debug, PartialEq)]
struct Task {
    id: u32,
    description: String,
}

fn parse_task(input: &str) -> Result<Task, String> {
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
        })
    } else {
        Err("Error: missing data".to_string())
    }
}

fn parse_tasks(tasks: &[&str]) -> (Vec<Task>, Vec<String>) {
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

fn main() {
    let tasks = vec![
        "1:Buy groceries",
        "2:Finish Rust exercise",
        "invalid task",
        "3:Read a book",
        "4:Review http://example.com",
        "5:Meeting at 10:30",
    ];

    println!("{:?}", parse_tasks(&tasks));

    println!("{:?}", parse_task(tasks[0]))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_valid_task() {
        let result = parse_task("1:Buy groceries");

        assert_eq!(
            result,
            Ok(Task {
                id: 1,
                description: "Buy groceries".to_string(),
            })
        );
    }

    #[test]
    fn parses_task_with_colons_in_description() {
        let result = parse_task("2:Meeting at 10:30");

        assert_eq!(
            result,
            Ok(Task {
                id: 2,
                description: "Meeting at 10:30".to_string(),
            })
        );
    }

    #[test]
    fn rejects_missing_separator() {
        let result = parse_task("1 Buy groceries");

        assert!(result.is_err());
    }

    #[test]
    fn rejects_non_numeric_id() {
        let result = parse_task("abc:Buy groceries");

        assert!(result.is_err());
    }

    #[test]
    fn rejects_empty_id() {
        let result = parse_task(":Buy groceries");

        assert!(result.is_err());
    }

    #[test]
    fn rejects_zero_id() {
        let result = parse_task("0:Buy groceries");

        assert!(result.is_err());
    }

    #[test]
    fn rejects_empty_description() {
        let result = parse_task("1:");

        assert!(result.is_err());
    }
}
