use clap::Clap;
use pickledb::{PickleDb, PickleDbDumpPolicy, SerializationMethod};
use std::error::Error;

#[derive(Clap)]
#[clap(author, version, about)]
struct UserArgs {
    /// Add task in 'id:value' format
    #[clap(short, long)]
    add: Option<String>,

    /// Mark task as done by ID
    #[clap(short, long)]
    done: Option<usize>,

    /// List incomplete tasks
    #[clap(short = 'l', long)]
    list: bool,

    /// Remove task by ID
    #[clap(short, long)]
    remove: Option<usize>,

    /// List completed tasks
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
        UserArgs::into_app().print_help()?;
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
    let mut db = get_database(db_name)?;

    let data: Vec<&str> = new_task.split(':').collect();
    if data.len() != 2 {
        println!("Data should be in 'id:value' format");
        return Ok(());
    }

    let id = data[0];
    let value = data[1];

    if db.exists(id) {
        println!("The task with ID '{}' already exists", id);
    } else {
        db.set(id, &value.to_string())?;
        println!("Task added: {} -> {}", id, value);
    }

    Ok(())
}

fn task_done(db_name: &str, task_id: usize, done_db_name: &str) -> Result<(), Box<dyn Error>> {
    let mut db = get_database(db_name)?;
    let mut done_db = get_database(done_db_name)?;

    match db.get::<String>(&task_id.to_string()) {
        Some(task_value) => {
            let done_tasks: Vec<String> = done_db.get(&task_id.to_string())
                .unwrap_or_else(|| Vec::new());
            done_db.set(&task_id.to_string(), &done_tasks)?;
            db.rem(&task_id.to_string())?;
            println!("Task marked as done and moved to 'done.db' successfully");
        }
        None => println!("Task with ID '{}' not found", task_id),
    }

    Ok(())
}

fn list_all_tasks(db_name: &str) -> Result<(), Box<dyn Error>> {
    let db = get_database(db_name)?;

    println!("Your TODO list:");
    for (key, value) in db.iter() {
        println!("{} -> {}", key, value);
    }

    Ok(())
}

fn list_done_tasks(db_name: &str) -> Result<(), Box<dyn Error>> {
    let done_db = get_database(db_name)?;

    println!("Completed tasks:");
    for (key, task_list) in done_db.iter() {
        for task in task_list {
            println!("{} -> {}", key, task);
        }
    }

    Ok(())
}

fn remove_task(db_name: &str, task_id: usize) -> Result<(), Box<dyn Error>> {
    let mut db = get_database(db_name)?;

    if db.exists(&task_id.to_string()) {
        db.rem(&task_id.to_string())?;
        println!("Task removed successfully");
    } else {
        println!("Task with ID '{}' not found", task_id);
    }

    Ok(())
}

fn get_database(db_name: &str) -> Result<PickleDb, Box<dyn Error>> {
    let db = PickleDb::new(
        db_name,
        PickleDbDumpPolicy::AutoDump,
        SerializationMethod::Json,
    )?;
    Ok(db)
}
