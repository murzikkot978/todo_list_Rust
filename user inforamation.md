# How to use

This program allows you to manage your to-dos effectively. You can add tasks with deadlines, sort your task list by due date, and assign a status to each task.


## Commands: 

* `cargo run` –for start program

* `--delete` –for delete one to-do from file 

* `--done` –for assign the status of done to the task 

* `--undone` –for assign the status of done to the task  

* `--due` –for add a deadline with date 

* `--list` –for show the entire to-do list 

* `--sort` –for sort the complete list of to-dos by date 

* `--id` –for give number of lines 


cargo run –we are using for starting our program 
For example, when you write this command, the program will ask to-do what you want to write in a file. 

After you can change the status of to-do with command --done and --undone. 

If you want add deadline for this task you can use command --due (YYYY-MM-DD). 

With this program you cand show the entire to-do list in the terminal, for this you need use --list. 

If you have lots of to-dos on your list with date you can sort this list by date with command --sort and know what you need to do first. 

## Exampeles

**Hove we need write our commands:**

* `cargo run` 

* `cargo run -- --delete --id (number of line)` 

* `cargo run -- --done/undone --id (number of line)`

* `cargo run -- --due (YYYY-MM-DD) --id (number of line)`

* `cargo run -- --list/sort`

We have one command. With this command you can look all commands: 

* `cargo run -- --help`

### [README](./README.md)

### [Video with examples](./Screencast%20from%202024-10-25%2014-16-05.mp4)

