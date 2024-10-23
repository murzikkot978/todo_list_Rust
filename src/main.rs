use clap::Parser;
use serde::{Deserialize, Serialize};
use serde_json::{self};
use std::fs::read_to_string;
use std::fs::{self};
use std::io;

#[derive(Serialize, Deserialize)]

struct Todo {
    contents: String,
}

#[derive(Parser)]
struct Flag {
    /// Write flag
    #[arg(long, short, default_value_t = 0)]
    delete: usize,
}

fn main() -> std::io::Result<()> {
    //Open file.
    let mut todos: Vec<Todo> = match read_to_string("todo.json") {
        Err(_) => Vec::new(),
        Ok(todos) => serde_json::from_str(&todos).expect("can't open file todo.json"),
    };

    let flag: Flag = Flag::parse();
    if flag.delete > 0 && flag.delete <= todos.len() {
        todos.remove(flag.delete - 1);

    
    } else {
        let mut message = String::new();
        println!("Give me your todo : ");
        io::stdin()
            .read_line(&mut message)
            .expect("Failed to read line");

        let message = message.trim();

        let todo = Todo {
            contents: message.to_string(),
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

    Ok(())
}
