use std::fs::OpenOptions;
use std::io;
use std::io::Write;

fn main() {
    let mut todo = String::new();
    println!("Give me your todo");
    io::stdin().read_line(&mut todo).expect("Read line failed");

    let todo = todo.trim();

    let mut todo_list = OpenOptions::new()
        .read(true)
        .write(true)
        .append(true)
        .create(true)
        .open("todo_list.txt")
        .expect("C'est pas posible");

    writeln!(todo_list, "{}", todo).expect("afsadva");
}
