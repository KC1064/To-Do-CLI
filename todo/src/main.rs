use std::fs::OpenOptions;
use std::io::{self, Read, Write};
use std::path::PathBuf;
use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};


#[derive(Parser, Debug)]
#[command(name = "todo", about = "A simple CLI to-do app")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    #[command(about = "Add a new to-do item")]
    Add {
        #[arg(name = "item")]
        item: String,
    },
    
    #[command(about = "Remove a to-do item")]
    Rm {
        #[arg(name = "item")]
        item: String,
    },
}

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

fn main() -> io::Result<()> {
    let cli = Cli::parse();
    let filename = PathBuf::from("todo_list.json");

    let mut todo_list = TodoList::load(&filename)?;

    match cli.command {
        Commands::Add { item } => {
            todo_list.add(item);
            println!("Item added.");
        },
        Commands::Rm { item } => {
            if todo_list.remove(&item) {
                println!("Item removed.");
            } else {
                println!("Item not found.");
            }
        },
    }

    todo_list.save(&filename)?;

    println!("Current to-do list:");
    todo_list.list();

    Ok(())
}

// use std::fs::OpenOptions;
// use std::io::{self, Read, Write};
// use std::path::PathBuf;
// use clap::{Parser, Subcommand};

// #[derive(Parser, Debug)]
// #[command(name = "todo", about = "A simple CLI to-do app")]
// struct Cli {
//     #[command(subcommand)]
//     command: Commands,
// }

// #[derive(Subcommand, Debug)]
// enum Commands {
//     #[command(about = "Manage your todo list")]
//     Todo {
//         #[command(subcommand)]
//         action: TodoActions,
//     },
// }

// #[derive(Subcommand, Debug)]
// enum TodoActions {
//     #[command(about = "Add a new to-do item")]
//     Add {
//         #[command(name = "item")]
//         item: String,
//     },
    
//     #[command(about = "Remove a to-do item")]
//     Rm {
//         #[command(name = "item")]
//         item: String,
//     },
// }

// #[derive(Debug)]
// struct TodoList {
//     items: Vec<String>,
// }

// impl TodoList {
//     fn new() -> Self {
//         Self { items: Vec::new() }
//     }

//     fn load(filename: &PathBuf) -> io::Result<Self> {
//         if filename.exists() {
//             let mut file = OpenOptions::new().read(true).open(filename)?;
//             let mut contents = String::new();
//             file.read_to_string(&mut contents)?;
//             let list: TodoList = serde_json::from_str(&contents)?;
//             Ok(list)
//         } else {
//             Ok(Self::new())
//         }
//     }

//     fn save(&self, filename: &PathBuf) -> io::Result<()> {
//         let contents = serde_json::to_string_pretty(self)?;
//         let mut file = OpenOptions::new().write(true).create(true).truncate(true).open(filename)?;
//         file.write_all(contents.as_bytes())?;
//         Ok(())
//     }

//     fn add(&mut self, item: String) {
//         self.items.push(item);
//     }

//     fn remove(&mut self, item: &str) -> bool {
//         if let Some(pos) = self.items.iter().position(|x| x == item) {
//             self.items.remove(pos);
//             true
//         } else {
//             false
//         }
//     }

//     fn list(&self) {
//         for (index, item) in self.items.iter().enumerate() {
//             println!("{}: {}", index + 1, item);
//         }
//     }
// }

// fn main() -> io::Result<()> {
//     let cli = Cli::parse();
//     let filename = PathBuf::from("todo_list.json");

//     let mut todo_list = TodoList::load(&filename)?;

//     match cli.command {
//         Commands::Todo { action } => match action {
//             TodoActions::Add { item } => {
//                 todo_list.add(item);
//                 println!("Item added.");
//             },
//             TodoActions::Rm { item } => {
//                 if todo_list.remove(&item) {
//                     println!("Item removed.");
//                 } else {
//                     println!("Item not found.");
//                 }
//             },
//         },
//     }

//     todo_list.save(&filename)?;

//     println!("Current to-do list:");
//     todo_list.list();

//     Ok(())
// }