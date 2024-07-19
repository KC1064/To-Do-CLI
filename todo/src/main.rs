use clap::{CommandFactory, Parser};
use pickledb::{PickleDb, PickleDbDumpPolicy, SerializationMethod};
use std::error::Error;
use std::path::Path;

/// CLI for managing your TODOs.
#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct UserArgs {
    /// Add a new task to your TODO list
    #[clap(short, long, value_parser)]
    add: Option<String>,

    /// Mark a task on your TODO list as completed
    #[clap(short, long, value_parser)]
    done: Option<usize>,

    /// List all of your incomplete tasks
    #[clap(short = 'l', long)]
    list: bool,

    /// Remove a task from your TODO list
    #[clap(short, long, value_parser)]
    remove: Option<usize>,

    /// List all tasks marked as done
    #[clap(short = 'c', long = "dl")]
    list_done: bool,

    /// Show help manual
    #[clap(short = 'h', long = "help")]
    help: bool,
}

fn main() -> Result<(), Box<dyn Error>> {
    let u_arg = UserArgs::parse();
    let todo_db_name = "cli_task.db";
    let done_db_name = "done.db";

    if u_arg.help {
        UserArgs::command().print_help()?;
    } else if let Some(task_to_add) = u_arg.add {
        add_task(todo_db_name, &task_to_add)?;
    } else if let Some(done_id) = u_arg.done {
        task_done(todo_db_name, done_id, done_db_name)?;
    } else if u_arg.list {
        list_all_tasks(todo_db_name)?;
    } else if let Some(remove_id) = u_arg.remove {
        remove_task(todo_db_name, remove_id)?;
    } else if u_arg.list_done {
        list_done_tasks(done_db_name)?;
    }

    Ok(())
}

fn add_task(db_name: &str, task: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut db = if Path::new(db_name).exists() {
        PickleDb::load(
            db_name,
            PickleDbDumpPolicy::AutoDump,
            SerializationMethod::Json,
        )?
    } else {
        PickleDb::new(
            db_name,
            PickleDbDumpPolicy::AutoDump,
            SerializationMethod::Json,
        )
    };

    let parts: Vec<&str> = task.split(':').collect();
    if parts.len() != 2 {
        return Err("Invalid task format".into());
    }

    db.set(parts[0], &parts[1].to_string())?;
    Ok(())
}

fn task_done(
    todo_db_name: &str,
    task_id: usize,
    done_db_name: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut todo_db = PickleDb::load(
        todo_db_name,
        PickleDbDumpPolicy::AutoDump,
        SerializationMethod::Json,
    )?;
    let mut done_db = if Path::new(done_db_name).exists() {
        PickleDb::load(
            done_db_name,
            PickleDbDumpPolicy::AutoDump,
            SerializationMethod::Json,
        )?
    } else {
        PickleDb::new(
            done_db_name,
            PickleDbDumpPolicy::AutoDump,
            SerializationMethod::Json,
        )
    };

    let key = task_id.to_string();
    if let Some(task) = todo_db.get::<String>(&key) {
        todo_db.rem(&key)?;
        let mut done_tasks = done_db.get::<Vec<String>>(&key).unwrap_or_default();
        done_tasks.push(task);
        done_db.set(&key, &done_tasks)?;
        Ok(())
    } else {
        Err("Task not found".into())
    }
}
fn list_all_tasks(db_name: &str) -> Result<String, Box<dyn Error>> {
    let mut output = String::new();

    if db_exists(db_name) {
        let db = PickleDb::load(
            db_name,
            PickleDbDumpPolicy::AutoDump,
            SerializationMethod::Json,
        )?;

        output.push_str("You have the following tasks:\n");
        let mut tasks: Vec<(String, String)> = db
            .iter()
            .map(|kv| {
                let key = kv.get_key().to_string();
                let value = kv.get_value::<String>().unwrap();
                (key, value)
            })
            .collect();

        // Sort tasks by their ID to ensure consistent order
        tasks.sort_by(|a, b| {
            a.0.parse::<usize>()
                .unwrap()
                .cmp(&b.0.parse::<usize>().unwrap())
        });

        for (id, task) in tasks {
            output.push_str(&format!("{}. {}\n", id, task));
        }
    } else {
        output.push_str("Database doesn't exist");
    }

    // Print the output to the console
    println!("{}", output);

    Ok(output)
}

fn list_done_tasks(db_name: &str) -> Result<String, Box<dyn Error>> {
    let mut output = String::new();

    if db_exists(db_name) {
        let db = PickleDb::load(
            db_name,
            PickleDbDumpPolicy::AutoDump,
            SerializationMethod::Json,
        )?;

        output.push_str("You have completed the following tasks:\n");
        println!("You have completed the following tasks:"); // Print to console

        for kv in db.iter() {
            let task_list = kv.get_value::<Vec<String>>().unwrap();
            for task in task_list {
                let line = format!("{}. {}\n", kv.get_key(), task);
                output.push_str(&line);
                print!("{}", line); // Print each line to console
            }
        }
    } else {
        output.push_str("'done.db' Database doesn't exist or no tasks completed yet");
        println!("'done.db' Database doesn't exist or no tasks completed yet"); // Print to console
    }

    Ok(output)
}

fn remove_task(db_name: &str, task_id: usize) -> Result<(), Box<dyn std::error::Error>> {
    let mut db = PickleDb::load(
        db_name,
        PickleDbDumpPolicy::AutoDump,
        SerializationMethod::Json,
    )?;
    let key = task_id.to_string();
    if db.exists(&key) {
        db.rem(&key)?;
        Ok(())
    } else {
        Err("Task not found".into())
    }
}


fn db_exists(db_name: &str) -> bool {
    Path::new(db_name).exists()
}

use std::fs;
// use std::path::Path;

const TODO_DB_NAME: &str = "cli_task.db";
const DONE_DB_NAME: &str = "done.db";

/// Helper function to set up a clean state for testing by deleting existing databases.
fn setup_test_db() {
    // Remove the TODO database if it exists
    if Path::new(TODO_DB_NAME).exists() {
        fs::remove_file(TODO_DB_NAME).expect("Failed to delete TODO database file");
    }

    // Remove the DONE database if it exists
    if Path::new(DONE_DB_NAME).exists() {
        fs::remove_file(DONE_DB_NAME).expect("Failed to delete DONE database file");
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use serial_test::serial;

    #[test]
    #[serial]
    fn test_add_task() {
        setup_test_db(); // Ensure clean state
        add_task(TODO_DB_NAME, "1:Test Task").expect("Failed to add task");

        let output = list_all_tasks(TODO_DB_NAME).expect("Failed to list tasks");
        assert!(
            output.contains("1. Test Task"),
            "Expected '1. Test Task' in output: {}",
            output
        );
    }

    #[test]
    #[serial]
    fn test_mark_task_done() {
        setup_test_db(); // Ensure clean state
        add_task(TODO_DB_NAME, "1:Test Task").expect("Failed to add task");
        task_done(TODO_DB_NAME, 1, DONE_DB_NAME).expect("Failed to mark task as done");

        let done_tasks = list_done_tasks(DONE_DB_NAME).expect("Failed to list done tasks");
        assert!(
            done_tasks.contains("1. Test Task"),
            "Expected '1. Test Task' in done tasks: {}",
            done_tasks
        );

        let todo_tasks = list_all_tasks(TODO_DB_NAME).expect("Failed to list tasks");
        assert!(
            !todo_tasks.contains("1. Test Task"),
            "Task '1. Test Task' should not be present in the TODO list: {}",
            todo_tasks
        );
    }

    #[test]
    #[serial]
    fn test_remove_task() {
        setup_test_db(); // Ensure clean state
        add_task(TODO_DB_NAME, "1:Test Task").expect("Failed to add task");
        remove_task(TODO_DB_NAME, 1).expect("Failed to remove task");

        let output = list_all_tasks(TODO_DB_NAME).expect("Failed to list tasks");
        assert!(
            !output.contains("1. Test Task"),
            "Task '1. Test Task' should be removed from the list: {}",
            output
        );
    }

    #[test]
    #[serial]
    fn test_invalid_task_format() {
        setup_test_db(); // Ensure clean state
        let result = add_task(TODO_DB_NAME, "1, New task"); // Invalid format

        assert!(
            result.is_err(),
            "Adding an invalid task format should return an error. Result: {:?}",
            result
        );
    }

    #[test]
    #[serial]

    fn test_list_tasks_and_done_tasks() {
        setup_test_db(); // Ensure clean state

        // Add tasks
        add_task(TODO_DB_NAME, "1:Test Task").expect("Failed to add task");
        add_task(TODO_DB_NAME, "2:Another Task").expect("Failed to add another task");

        // Mark a task as done
        task_done(TODO_DB_NAME, 1, DONE_DB_NAME).expect("Failed to mark task as done");

        // Verify TODO list
        let todo_tasks = list_all_tasks(TODO_DB_NAME).expect("Failed to list tasks");
        assert!(
            todo_tasks.contains("2. Another Task"),
            "Task '2. Another Task' should be present in the TODO list. Output: {}",
            todo_tasks
        );
        assert!(
            !todo_tasks.contains("1. Test Task"),
            "Task '1. Test Task' should not be present in the TODO list. Output: {}",
            todo_tasks
        );

        // Verify DONE list
        let done_tasks = list_done_tasks(DONE_DB_NAME).expect("Failed to list done tasks");
        assert!(
            done_tasks.contains("1. Test Task"),
            "Task '1. Test Task' should be present in the DONE list. Output: {}",
            done_tasks
        );
        assert!(
            !done_tasks.contains("2. Another Task"),
            "Task '2. Another Task' should not be present in the DONE list. Output: {}",
            done_tasks
        );
    }

    #[test]
    #[serial] // Ensure tests are run sequentially
    fn test_remove_non_existent_task() {
        setup_test_db(); // Ensure clean state

        // Adding tasks
        add_task(TODO_DB_NAME, "1:Test Task").expect("Failed to add task");
        add_task(TODO_DB_NAME, "2:Another Task").expect("Failed to add another task");

        // Attempt to remove a non-existent task
        let result = remove_task(TODO_DB_NAME, 99); // Task ID 99 does not exist

        // Verify that the result is an error and it matches the expected error message
        assert!(
            result.is_err(),
            "Expected an error when removing a non-existent task, but got: {:?}",
            result
        );

        if let Err(e) = result {
            assert_eq!(
                e.to_string(),
                "Task not found",
                "Error message should be 'Task not found'"
            );
        }

        // Verify that the TODO list still contains the tasks that were added
        let todo_tasks = list_all_tasks(TODO_DB_NAME).expect("Failed to list tasks");
        assert!(
            todo_tasks.contains("1. Test Task"),
            "The TODO list should contain 'Test Task'"
        );
        assert!(
            todo_tasks.contains("2. Another Task"),
            "The TODO list should contain 'Another Task'"
        );
    }
}
