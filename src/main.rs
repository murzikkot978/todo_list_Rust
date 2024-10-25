use chrono::NaiveDate;
use clap::Parser;
use serde::{Deserialize, Serialize};
use serde_json::{self};
use std::fs::read_to_string;
use std::fs::{self};
use std::io;

#[derive(Serialize, Deserialize)]
struct Todo {
    contents: String,
    status: bool,
    deadline: Option<NaiveDate>,
}

// Define the Flag struct that will parse command-line flags using Clap
#[derive(Parser)]
struct Flag {
    #[arg(long)]
    delete: bool,

    #[arg(long)]
    done: bool,

    #[arg(long)]
    undone: bool,

    #[arg(long)]
    due: Option<String>,

    #[arg(long)]
    list: bool,

    #[arg(long)]
    sort: bool,

    #[arg(long, default_value_t = 0)]
    id: usize,
}

fn main() {
    // Open and read the todo file or create a new
    let mut todos: Vec<Todo> = match read_to_string("todo.json") {
        Err(_) => Vec::new(),
        Ok(todos) => serde_json::from_str(&todos).expect("can't open file todo.json"),
    };

    // Parse command-line arguments into the Flag struct
    let flag: Flag = Flag::parse();
    if flag.delete {
        todos.remove(flag.id - 1);
        println!("Tâche supprimée.");

    // Set the status of the todo to true
    } else if flag.done {
        todos[flag.id - 1].status = true;

    // Set the status of the todo to false
    } else if flag.undone {
        todos[flag.id - 1].status = false;

    //Parse the date and set it as the deadline for the todo
    } else if let Some(date_string) = flag.due {
        match NaiveDate::parse_from_str(&date_string, "%Y-%m-%d") {
            Ok(date) => {
                todos[flag.id - 1].deadline = Some(date);
            }
            Err(_) => {
                println!("Invalid date format! Please use YYYY-MM-DD.");
            }
        }

    // Show the entire list of todo with status
    } else if flag.list {
        for (i, list_todo) in todos.iter().enumerate() {
            let status = if list_todo.status { "Done" } else { "Not Done" };

            println!("{}. {} = {}", i + 1, list_todo.contents, status);
        }

    // Sort the entire list of todo by date
    } else if flag.sort {
        todos.sort_by(|a, b| a.deadline.cmp(&b.deadline));
    } else {
        let mut message = String::new();
        println!("Give me your todo : ");
        io::stdin()
            .read_line(&mut message)
            .expect("Failed to read line");

        // Trim whitespace from the input
        let message = message.trim();

        let todo = Todo {
            contents: message.to_string(),
            status: false,
            deadline: None,
        };

        todos.push(todo);
    }

    // Write the updated list of todos back to the "todo.json"
    fs::write(
        "todo.json",
        serde_json::to_string(&todos).expect("can't write to file!"),
    )
    .expect("can't write file");
}
