use clap::{Parser, CommandFactory};
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

fn add_task(db_name: &str, new_task: &String) -> Result<(), Box<dyn Error>> {
    let mut db = if db_exists(db_name) {
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

    let data = new_task.split(":").collect::<Vec<&str>>();

    if data.len() == 2 {
        if let Some(value) = db.get::<String>(data[0]) {
            println!("The value of id: {} already exists: {}", data[0], value);
        } else {
            db.set(data[0], &data[1].to_string())?;
            println!(
                "The value of id: {} is: {}",
                data[0],
                db.get::<String>(data[0]).unwrap()
            );
        }
    } else {
        println!("Data should be in 'id:value' format");
    }

    Ok(())
}

fn task_done(db_name: &str, old_task: usize, done_db_name: &str) -> Result<(), Box<dyn Error>> {
    if db_exists(db_name) {
        let mut db = PickleDb::load(
            db_name,
            PickleDbDumpPolicy::AutoDump,
            SerializationMethod::Json,
        )?;

        let value = db.get::<String>(&old_task.to_string());

        if let Some(task_value) = value {
            let mut done_db = if db_exists(done_db_name) {
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

            let key = old_task.to_string();
            let mut task_list = done_db.get::<Vec<String>>(&key).unwrap_or_else(|| Vec::new());
            task_list.push(task_value);
            done_db.set(&key, &task_list)?;

            db.rem(&key)?;

            println!("Task is marked done and moved to 'done.db' successfully");
        } else {
            println!("Task is not found");
        }
    } else {
        println!("Database doesn't exist");
    };

    Ok(())
}

fn list_all_tasks(db_name: &str) -> Result<(), Box<dyn Error>> {
    if db_exists(db_name) {
        let db = PickleDb::load(
            db_name,
            PickleDbDumpPolicy::AutoDump,
            SerializationMethod::Json,
        )?;

        println!("You have the following tasks: ");
        for kv in db.iter() {
            println!("{}. {}", kv.get_key(), kv.get_value::<String>().unwrap());
        }
    } else {
        println!("Database doesn't exist");
    };

    Ok(())
}

fn list_done_tasks(db_name: &str) -> Result<(), Box<dyn Error>> {
    if db_exists(db_name) {
        let db = PickleDb::load(
            db_name,
            PickleDbDumpPolicy::AutoDump,
            SerializationMethod::Json,
        )?;

        println!("You have completed the following tasks: ");
        for kv in db.iter() {
            let task_list = kv.get_value::<Vec<String>>().unwrap();
            for task in task_list {
                println!("{}. {}", kv.get_key(), task);
            }
        }
    } else {
        println!("'done.db' Database doesn't exist or no tasks completed yet");
    };

    Ok(())
}

fn remove_task(db_name: &str, task_id: usize) -> Result<(), Box<dyn Error>> {
    if db_exists(db_name) {
        let mut db = PickleDb::load(
            db_name,
            PickleDbDumpPolicy::AutoDump,
            SerializationMethod::Json,
        )?;

        let success = db.rem(&task_id.to_string())?;

        if success {
            println!("Task is removed successfully");
        } else {
            println!("Task is not found");
        }
    } else {
        println!("Database doesn't exist");
    };

    Ok(())
}

fn db_exists(db_name: &str) -> bool {
    Path::new(db_name).exists()
}
