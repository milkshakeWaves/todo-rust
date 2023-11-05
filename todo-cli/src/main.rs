use std::io;
use std::io::Write;

use repository::{InMemoryRepository, TodoRepository};
use todo_cli::Todo;

use crate::repository::ShowTodosOptions;

mod lib;
mod repository;

fn main() {
    let mut todo_repo: InMemoryRepository = InMemoryRepository::new();

    loop {
        println!("Enter a command!");
        print!(">  ");
        io::stdout().flush().unwrap();

        let mut input: String = String::new();

        if let Err(_) = io::stdin().read_line(&mut input) {
            println!("Error reading input");
            continue;
        }

        let input: &str = input.trim();
        let parts: Vec<&str> = input.split(' ').collect();

        match parts[0] {
            "create" => {
                println!("Creating a new todo...");
                match todo_repo.create_todo(&parts[1..].join(" ")) {
                    Some(created_todo) => println!("Todo {:?} successfully created!", created_todo),
                    _ => println!("Error: cannot create todo!")
                }
                continue;
            }
            "edit" => {
                println!("edit a todo");
                continue;
            }
            "delete" => {
                println!("edit a todo");
                continue;
            }
            "done" => {
                println!("done a todo");
                continue;
            }
            "show" => {
                println!("Show all the todos");
                let all_todos: Vec<&Todo> = todo_repo.show_todos(&ShowTodosOptions::All);
                println!("############################################");
                println!("{:?}", all_todos);
                println!("############################################");
                continue;
            }
            "exit" => {
                println!("Exiting the program...\n");
                break;
            }
            _ => {
                println!("Command not recognized!\n");
            }
        }
    }
}
