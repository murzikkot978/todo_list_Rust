use serde::{Deserialize, Serialize};
use serde_json::{self};
use std::fs::read_to_string;
use std::fs::{self};
use std::io;

#[derive(Serialize, Deserialize)]

struct Todo {
    contents: String,
}

fn main() -> std::io::Result<()> {
    let mut message = String::new();
    println!("Give me your todo : ");
    io::stdin()
        .read_line(&mut message)
        .expect("Failed to read line");

    let message = message.trim();

    let todo = Todo {
        contents: message.to_string(),
    };

    //Open file.
    let mut todos: Vec<Todo> = match read_to_string("todo.json") {
        Err(_) => Vec::new(),
        Ok(todos) => serde_json::from_str(&todos).expect("can't open file todo.json"),
    };

    //Check if the user typed --delete.

    if message.contains("--delete") {
        //Find the last value.
        let test = message.split(" ").last();
        let nombre_line: usize = test.expect("Cannot split").parse().unwrap();

        todos.remove(nombre_line - 1);
    } else {
        todos.push(todo);
    }

    fs::write(
        "todo.json",
        serde_json::to_string(&todos).expect("can't write to file!"),
    )
    .expect("can't write file");

    Ok(())
}
