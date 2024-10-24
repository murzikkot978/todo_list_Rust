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

    #[arg(long, default_value_t = 0)]
    id: usize
}

fn main() {
    //Open file.
    let mut todos: Vec<Todo> = match read_to_string("todo.json") {
        Err(_) => Vec::new(),
        Ok(todos) => serde_json::from_str(&todos).expect("can't open file todo.json"),
    };

    let flag: Flag = Flag::parse();
    if flag.delete {
        todos.remove(flag.id - 1);
        println!("Tâche supprimée.");
    } else if flag.done{
        todos[flag.id - 1].status = true;
    } else if flag.undone {
        todos[flag.id - 1].status = false;
    } else if let Some(date_string) = flag.due {
    

        match NaiveDate::parse_from_str(&date_string, "%Y-%m-%d") {
            Ok(date) => {
                todos[flag.id - 1].deadline = Some(date);
            }
            Err(_) => {
                println!("Invalid date format! Please use YYYY-MM-DD.");
            }
        }
    } else {
        let mut message = String::new();
        println!("Give me your todo : ");
        io::stdin()
            .read_line(&mut message)
            .expect("Failed to read line");

        let message = message.trim();

        let todo = Todo {
            contents: message.to_string(),
            status: false,
            deadline: None,
        };

        if message.contains("--delete") {
            //Find the last value.
            let test = message.split(" ").last();
            let nombre_line: usize = test.expect("Cannot split").parse().unwrap();

            todos.remove(nombre_line - 1);
        } else {
            todos.push(todo);
        }
    }

    fs::write(
        "todo.json",
        serde_json::to_string(&todos).expect("can't write to file!"),
    )
    .expect("can't write file");
}
