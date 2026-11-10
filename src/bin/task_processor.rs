#[path = "task_processor/mod.rs"]
mod task_processor;

use crate::task_processor::status::Status;
use crate::task_processor::task::{Summary, Task};
use crate::task_processor::task_list::{TaskList, print_items};

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
            status: Status::Pending,
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

fn find_task(tasks: &[Task], id: u32) -> Option<&Task> {
    tasks.iter().find(|task| task.id == id)
}

fn complete_task(task: &mut Task) {
    task.status = Status::Complete
}

fn is_complete(task: &Task) -> bool {
    // match &task.status {
    //     Status::Complete => true,
    //     Status::InProgress => false,
    //     Status::Pending => false,
    // }
    matches!(task.status, Status::Complete)
}

fn start_task(task: &mut Task) {
    match &task.status {
        Status::Pending => task.status = Status::InProgress,
        _ => {}
    }
}

fn filter_by_status<'a>(tasks: &'a [Task], status: &Status) -> Vec<&'a Task> {
    tasks.iter().filter(|task| task.status == *status).collect()
}

fn print_report(tasks: &[Task], errors: &[String]) {
    println!("{} valid tasks", tasks.len());
    for task in tasks {
        println!(
            "id: {} description: {} status {:?}",
            task.id, task.description, task.status
        );
    }
    println!("{} errors", errors.len());
    for error in errors {
        println!("{}", error);
    }
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
    let task = parse_task(tasks[0]);
    let mut parsed = parse_tasks(&tasks);

    println!("{:?}", parse_tasks(&tasks));
    println!("{:?}", task);
    println!("{:?}", is_complete(&parsed.0[0]));
    println!("{:?}", find_task(&parsed.0, 4));
    complete_task(&mut parsed.0[0]);
    start_task(&mut parsed.0[1]);
    println!("{:?}", filter_by_status(&parsed.0, &Status::Pending));
    println!("{}", parsed.0[0].summary());
    print_report(&parsed.0, &parsed.1);

    match task {
        Ok(t) => {
            let mut list = TaskList::<Task>::new();
            list.add(t);
            println!("{} {}", list.len(), list.is_empty())
        }
        Err(e) => println!("{}", e),
    }

    print_items(&parsed.0);

    let mut list = TaskList::<Task>::new();
    for task in parsed.0 {
        list.add(task);
    }
    list.print_summaries();
    list.print_debug_summaries();
    println!("{:?}", list.summaries());
    println!("{:?}", list.find_by_summary("rust"));
    println!("{:?}", list.find_cloned_by_summary("rust"));
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
                status: Status::Pending,
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
                status: Status::Pending,
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
