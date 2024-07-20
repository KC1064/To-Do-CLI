# To-DO-CLI-APP

## APPROACH
- To create this project I used clap crate for parsing command line arguments and pickleDB for storing data in key-value format that are stored in json format. 
- It performs adding of tasks, removing of tasks, marking tasks done, showing current todo lists and showing completed tasks.
- To manage that we used to databases

**List of Commands**
- `cargo run -- -a "1: New Task"`: To add task.
- `cargo run -- -r 1`: To remove task.
- `cargo run -- -d 1`: To mark the task completed.
- `cargo run -- -l`: To show the current todo list
- `cargo run -- --dl`: To show the completed task list.

## IMPORT OF LIBRARIES AND CRATESSS

1. **`use clap::{CommandFactory, Parser};`**:

   - **`CommandFactory`**: To create command-line applications.
   - **`Parser`**: To parse command-line arguments.

2. **`use pickledb::{PickleDb, PickleDbDumpPolicy, SerializationMethod};`**:
   - **`PickleDb`**: Struct to interact with the database.
   - **`PickleDbDumpPolicy`**: Policy for saving the database (e.g., `AutoDump`).
   - **`SerializationMethod`**: To store the data in json format.

**NOTE:** The `pickledb` crate is a lightweight key-value store using a simple serialization format.

3. **`use std::error::Error;`**:

   - Imports the `Error` trait for rust's standard library.
   - Used with the `Result` type to handle errors in standardized way.

4. **`use std::path::Path;`**:
   - Imports the `Path` struct from Rust's standard library.
   - It is a struct that provides a way to work with file system paths.
   - It allows to manipulate and inspect paths in a platform-independent way.

### Summary

- **`clap`**: For command-line argument parsing.
- **`pickledb`**: For a simple, persistent key-value database.
- **`std::error::Error`**: For error handling.
- **`std::path::Path`**: For file system path operations.

## COMMANDS

```rust
#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct UserArgs {

    #[clap(short, long, value_parser)]
    add: Option<String>,

    #[clap(short, long, value_parser)]
    done: Option<usize>,

    #[clap(short = 'l', long)]
    list: bool,

    #[clap(short, long, value_parser)]
    remove: Option<usize>,

    #[clap(short = 'c', long = "dl")]
    list_done: bool,

    #[clap(short = 'h', long = "help")]
    help: bool,
}
```

- Here the `#[derive(Parser)]` is macro that derives the `Parser` trait for `UserArgs` struct, Which helps to parse CLA.

- `#[clap(author, version, about, long_about = None)]` these are just meta-information, it is displayed when using `--help` flag or due to parsing error.

## Struct UserArgs

- This struct defines the command-line arguments and flags for the application using the `clap` crate.

- **add**,**remove** and **done** : All these have optional field that means if passed they would take subsequent data types as assigned to them.

- **list**,**list_done** and **help** : These are boolean flags that can be set true when parsed into CL.

**_Purpose of this struct_**: This struct allows the application to interpret and handle various command-line inputs for task management effectively.

## TODO ACTIONS:

### `add_task` function:

```rust
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
```

- Here I check if the database file exists, if it exists, we load using `load` method.
- If the database file does not exist, we create a new database using `new` method.
- Now the `PickleDbDumpPolicy::AutoDump` ensures that the database us saved automatically after each write operation.
- `SerializationMethod::Json` specifies that the data should be stored in JSON format.
- To add task a specific format is mentioned, by spliting the string using `:` delimiter.
- `db.set(parts[0], &parts[1].to_string())?` stores the task in the database in key-value format.
- `?` operator is used to handle errors, if any, and return a `Result`

### `task_done` function:

```rust fn task_done(
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
```
- Does the same to check db is present or not.
- It retrieves the task with the given ID from the to-do database.
- If the task is found, it proceeds to the next step otherwise, it returns an error.
- It removes the task from current todo db to done db using `set` method.

### `list_all_tasks`  function:

```rust
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

    // To print to console
    println!("{}", output);

    Ok(output)
}
```
- Again the same as above to check db.
- It collects all tasks from the database into a vector of tuples (ID, task description).
- It sorts the vector by ID and prints the tasks to the console.
- constructs a formatted string listing all tasks, which is printed to the console and returned.

### `list_done_task` functon

```rust
fn list_done_tasks(db_name: &str) -> Result<String, Box<dyn Error>> {
    let mut output = String::new();

    if db_exists(db_name) {
        let db = PickleDb::load(
            db_name,
            PickleDbDumpPolicy::AutoDump,
            SerializationMethod::Json,
        )?;

        output.push_str("You have completed the following tasks:\n");
        println!("You have completed the following tasks:");

        for kv in db.iter() {
            let task_list = kv.get_value::<Vec<String>>().unwrap();
            for task in task_list {
                let line = format!("{}. {}\n", kv.get_key(), task);
                output.push_str(&line);
                print!("{}", line);
            }
        }
    } else {
        output.push_str("'done.db' Database doesn't exist or no tasks completed yet");
        println!("'done.db' Database doesn't exist or no tasks completed yet"); 
    }

    Ok(output)
}
```
- Same as above function to check database exists or not.
- Then to print all the list we iterate through the done database.
- For each entry (which is a list of completed tasks under a specific ID), it constructs a formatted string and prints it to the console.
- It also builds the output string, which is returned.

### `remove_task` function

```rust
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
```

- First it checks if the task with given ID is present in DB.
- If it exists it removes the key-item using `rem` method.
- If it doesn't exist it returns an error message.

## MAIN FUNCTION:

- Parses CL arguments, performs actions based on these arguments, and handles potential errors.
- I have also defined two database one to store current todo task and another to store the completed tasks.
- Now the if-else used to match the argument from CL and perform the respective actions.
- The function returns `Ok(())` if everything executes successfully. The Result type is used for error handling, and any error that occurs during the execution of the function.


## TEST CASE:
- `use super::*` imports all items from the outer scope.
- `use serial_test::serial` imports the ***serial*** attribute from ***serial_test*** create to ensure the tests run serially.
### Different cases
-  ***Verifies that a task can be successfully added to the TODO list and that it appears in the task list.***
- ***Tests marking a task as done and ensures it is moved from the TODO list to the DONE list.***
- ***Verifies that a task can be removed from the TODO list.***
- ***Tests handling of an invalid task format.***
- ***Tests both listing tasks and done tasks after adding and marking tasks.***
- ***Tests the behavior when attempting to remove a non-existent task.***