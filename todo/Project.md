## PART-1: IMPORTS

```rust
use std::fs::OpenOptions;
use std::io::{self, Read, Write};
use std::path::PathBuf;
use clap::{Parser, Subcommand, Args};
use serde::{Deserialize, Serialize};
```

- `std::fs::OpenOptions`: Provides options for opening files in different modes like read, write and append
- `std::io::{self, Read, Write}`: Imports self from module io, Read and Write i.e a trait provides methods for reading and writing in source and detsination.
- `std::path::PathBuf`: Provides a method for building and modifying paths.
- `clap::{Parser, Subcommand, Args}`: Imported from clap where parser for parsingg cmd line args, Subcommand a derive macro for Cmd line app, Args a macro to define arguments for a subcommand.
- `serde::{Deserialize, Serialize}`: Imported from serde which allows serializing and deserializing of to-do list to and from a JSON file.

## PART-2: DEFINING SUBCOMMANDS AND ACTIONS

```rust
#[derive(Parser)]
#[command(name = "todo", about = "A simple to-do list CLI", version = "1.0")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}
```

- **`#[derive(Parser)]`**: Macro derives the `Parser` trait for the `Cli` struct, allowing to parse command-line arguments.
- **`#[command(name = "todo", about = "A simple to-do list CLI", version = "1.0")]`**: Macro provides metadata for the command-line application like name, about, versions etc.

- **`struct Cli`**: Main struct to parse command-line arguments. It contains a **`command: Option<Commands>`**: represents the subcommands of the CLI. The `Option` type can be `None` (no command provided) or `Some(Commands)`.

```rust
#[derive(Subcommand)]
enum Commands {
    #[command(about = "Add a new to-do item")]
    Add(AddArgs),

    #[command(about = "Remove a to-do item")]
    Rm(RmArgs),

    #[command(about = "List all to-do items")]
    List,
}
```

- **`#[derive(Subcommand)]`**: macro derives the `Subcommand` trait for the `Commands` enum.

- **`enum Commands`**: defines the possible subcommands for the CLI
- **`Add(AddArgs)`**: `add` subcommand, takes arguments defined by the `AddArgs` struct.
- **`Rm(RmArgs)`**: `rm` subcommand, takes arguments defined by the `RmArgs` struct.
- **`List`**: Represents the `list` subcommand, which lists all to-do items.

```rust
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
```

- **`#[derive(Args, Debug)]`**: macro that derives the `Args` and `Debug` traits for the `AddArgs` and `RmArgs` struct.
- The `Args` trait allows the struct to parse arguments for a subcommand, and the `Debug` trait provides debugging capabilities.

## PART-3: TO-DO-LIST IMPLEMENTATION

```rust
#[derive(Debug, Serialize, Deserialize)]
struct TodoList {
    items: Vec<String>,
}
```

- **`#[derive(Debug, Serialize, Deserialize)]`**: Automatically implements the `Debug`, `Serialize`, and `Deserialize` traits for the `TodoList` struct.
- `Serialize` allows converting the struct to a JSON string.
- `Deserialize` allows creating the struct from a JSON string.
- **`items: Vec<String>`**: A vector that holds the list of to-do items as strings.

```rust
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
```

1. **`fn new() -> Self`**:
   - Creates a new instance of `TodoList` with an empty vector of items.
2. **`fn load(filename: &PathBuf) -> io::Result<Self>`**:

   - Loads a `TodoList` from a file specified by `filename`.
   - Returns an `io::Result` containing the loaded `TodoList` or an error.

3. **`fn save(&self, filename: &PathBuf) -> io::Result<()>`**:

   - Saves the `TodoList`
   - Serializes the `TodoList` to a pretty-printed JSON string using `serde_json::to_string_pretty`.
   - Opens the file for writing.
   - Writes the JSON string to the file.
   - Returns an `io::Result` indicating success or failure.

4. **`fn add(&mut self, item: String)`**:

   - Adds a new item to the `TodoList`.
   - Appends the item to the `items` vector.

5. **`fn remove(&mut self, item: &str) -> bool`**:

   - Removes an item from the `TodoList`.
   - Searches for the item in the `items` vector.
   - If found, removes the item and returns `true`.
   - If not found, returns `false`.

6. **`fn list(&self)`**:
   - Lists all items in the `TodoList`.
   - Prints each item with its index using `enumerate`.

#### NOTE: `enumerate` : creates an iterator that yields pairs of elements.

## PART-4: MAIN FUNCTION

```rust
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

```

1. ```let cli = Cli::parse();```

   - Uses `parse()` function to parse command-line arguments using the Clap library. 

2. ```let filename = PathBuf::from("todo_list.json");```

   - Defines a `PathBuf` named `filename` pointing to `"todo_list.json"`, the JSON file where the to-do list will be stored.

3. ```let mut todo_list = TodoList::load(&filename)?;```

   - Loads `TodoList` from the JSON file specified by `filename` using the `load` method of `TodoList`. If successful, `todo_list` holds the loaded to-do list.

4. 
```rust
   if let Some(command) = cli.command {
       match command {
           // Handling Add command
           Commands::Add(args) => {
               todo_list.add(args.item);
               println!("Item added.");
           },
           // Handling Rm command
           Commands::Rm(args) => {
               if todo_list.remove(&args.item) {
                   println!("Item removed.");
               } else {
                   println!("Item not found.");
               }
           },
           // Handling List command
           Commands::List => {
               println!("Current to-do list:");
               todo_list.list();
           },
       }
   } else {
       println!("No command provided.");
   }
   ```

   - Checks if a subcommand matches against the different variants of `Commands` (`Add`, `Rm`, `List`) to execute the corresponding actions.
   - If not valid subcommand, it prints `"No command provided."`.

5. ```todo_list.save(&filename)?;```

   - Saves the updated `todo_list` back to the JSON file specified by `filename` using the `save`.

6. ```Ok(())```
   - `Ok(())` is returned to indicate that the function executed successfully.
   - `()` represents an empty tuple or unit.

#### NOTE: `?` used for hadnling errors in functions that returns a `Result`. It means if  the function returns an error, the program will exit with an error message.