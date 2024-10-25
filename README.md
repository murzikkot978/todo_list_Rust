# How to start todo list Rust

With this program you can write your to-do in the file JSON and read this to-do.


First, you need to download the Rast programming language.

To do this, you need to open a terminal and enter this command:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
```

To download this program, you need to follow this link:

```bash
git clone git@github.com:murzikkot978/todo_list_Rust.git
```

Then you need to click on the green code button on the right side of the screen. And then you need to select Local; SSH; and download the zip file.

The next step is to find our program using the terminal.

To do this, you'll need to route to the program using the command:

```bash
cd todo_list_Rust
```

**Information on use in the** [**user information.**](./user%20inforamation.md)

## Documentation

At the beginning of the file, two structures were written, the first structure contained what needed to be written to the file, the second structure contained what flags were used.

At the beginning of the lesson, the program opens the file and reads it; if the file does not exist, it creates a new one.

Then it checks what the user has written and checks the checkboxes. If the user has written only `cargo run`, the program asks to-do.

Finally, the program writes everything to thr file **JSON**.