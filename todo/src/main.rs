use std::fs::OpenOptions;
use std::io::{self, Read, Write};
use std::path::PathBuf;
use clap::{Parser, Subcommand, Args};
use serde::{Deserialize, Serialize};

#[derive(Parser)]
#[command(name = "todo", about = "A simple to-do list CLI", version = "1.0")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = "Add a new to-do item")]
    Add(AddArgs),
    
    #[command(about = "Remove a to-do item")]
    Rm(RmArgs),
    
    #[command(about = "List all to-do items")]
    List,
}

#[derive(Args, Debug)]
struct AddArgs {
    #[arg(name = "item")]
    item: String,
}

#[derive(Args, Debug)]
struct RmArgs {
    #[arg(name = "item")]
    item: String,
}

// Part 2
#[derive(Debug, Serialize, Deserialize)]
struct TodoList {
    items: Vec<String>,
}

impl TodoList {
    fn new() -> Self {
        Self { items: Vec::new() }
    }

    fn load(filename: &PathBuf) -> io::Result<Self> {
        if filename.exists() {
            let mut file = OpenOptions::new().read(true).open(filename)?;
            let mut contents = String::new();
            file.read_to_string(&mut contents)?;
            let list: TodoList = serde_json::from_str(&contents)?;
            Ok(list)
        } else {
            Ok(Self::new())
        }
    }

    fn save(&self, filename: &PathBuf) -> io::Result<()> {
        let contents = serde_json::to_string_pretty(self)?;
        let mut file = OpenOptions::new().write(true).create(true).truncate(true).open(filename)?;
        file.write_all(contents.as_bytes())?;
        Ok(())
    }

    fn add(&mut self, item: String) {
        self.items.push(item);
    }

    fn remove(&mut self, item: &str) -> bool {
        if let Some(pos) = self.items.iter().position(|x| x == item) {
            self.items.remove(pos);
            true
        } else {
            false
        }
    }

    fn list(&self) {
        for (index, item) in self.items.iter().enumerate() {
            println!("{}: {}", index + 1, item);
        }
    }
}

// Part 3
fn main() -> io::Result<()> {
    let cli = Cli::parse();
    let filename = PathBuf::from("todo_list.json");

    let mut todo_list = TodoList::load(&filename)?;

    if let Some(command) = cli.command {
        match command {
            Commands::Add(args) => {
                todo_list.add(args.item);
                println!("Item added.");
            },
            Commands::Rm(args) => {
                if todo_list.remove(&args.item) {
                    println!("Item removed.");
                } else {
                    println!("Item not found.");
                }
            },
            Commands::List => {
                println!("Current to-do list:");
                todo_list.list();
            },
        }
    } else {
        println!("No command provided.");
    }

    todo_list.save(&filename)?;

    Ok(())
}
