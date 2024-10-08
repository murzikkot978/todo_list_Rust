use std::fs;
use std::fs::read_to_string;
use std::io;

fn main() -> std::io::Result<()> {
    let mut todo = String::new();
    println!("Give me your todo : ");
    io::stdin()
        .read_line(&mut todo)
        .expect("Failed to read line");

    let todo = todo.trim();
    //Open file.
    let mut todos: Vec<String> = match read_to_string("todo_list.txt") {
        Err(_) => Vec::new(),
        Ok(todo_list) => todo_list.lines().map(String::from).collect(),
    };

    //Check if the user typed --delete.

    if todo.contains("--delete") {
        //Find the last value.
        let test = todo.split(" ").last();
        let nombre_line: usize = test.expect("Err").parse().unwrap();

        todos.remove(nombre_line - 1);
    } else {
        todos.push(todo.to_string());
    }

    fs::write("todo_list.txt", todos.join("\n")).expect("can't write to file!");

    Ok(())
}
