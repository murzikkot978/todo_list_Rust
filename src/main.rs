use std::fs::File;
use std::io;
use std::io::Result;
use std::io::Write;

fn main() -> Result<()> {
    let mut todo = String::new();
    println!("Give me your todo");
    io::stdin().read_line(&mut todo).expect("Read line failed");

    let mut todo_list = File::create("todo_list.txt")?;

    let _ = todo_list.write_all(todo.as_bytes());
    todo_list.flush()?;

    Ok(())
}
