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
}

#[derive(Parser)]
struct Flag {
    /// Write nombre line
    #[arg(long, default_value_t = 0)]
    delete: usize,

    /// Write nombre line
    #[arg(long, default_value_t = 0)]
    done: usize,

    /// Write nombre line
    #[arg(long, default_value_t = 0)]
    undone: usize,


}

fn main() {
    //Open file.
    let mut todos: Vec<Todo> = match read_to_string("todo.json") {
        Err(_) => Vec::new(),
        Ok(todos) => serde_json::from_str(&todos).expect("can't open file todo.json"),
    };

    let flag: Flag = Flag::parse();
    if flag.delete > 0 && flag.delete <= todos.len() {
        todos.remove(flag.delete - 1);
        println!("Tâche supprimée.");
    } else if flag.done > 0 && flag.done <= todos.len() {
        todos[flag.done - 1].status = true;
    } else if flag.undone > 0 && flag.undone <= todos.len() {
        todos[flag.undone -1].status = false;
    }else {
        let mut message = String::new();
        println!("Give me your todo : ");
        io::stdin()
            .read_line(&mut message)
            .expect("Failed to read line");

        let message = message.trim();

        let todo = Todo {
            contents: message.to_string(),
            status: false,
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
