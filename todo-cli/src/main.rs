use std::io;
use std::io::Write;

use repository::{InMemoryRepository, TodoRepository};
use todo_cli::Todo;

use crate::repository::ShowTodosOptions;

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
                let inserted_todo = todo_repo.create_todo(&parts[1..].join(" "));
                println!("Todo {:?} successfully created!", inserted_todo);
                continue;
            }
            "edit" => {
                println!("edit a todo");
                continue;
            }
            "delete" => {
                match parts[1].parse::<u32>() {
                    Ok(id) => match todo_repo.delete_todo(id) {
                        Some(todo) => println!("Todo {:?} successfully deleted!", todo),
                        _ => println!("Error: todo n°{} does not exist", id),
                    },
                    Err(e) => {
                        // Conversion failed
                        println!("Failed to parse number: {}", e);
                    }
                }
                continue;
            }
            "done" => {
                match parts[1].parse::<u32>() {
                    Ok(id) => {
                        todo_repo.mark_as_done(id);
                        println!("Todo n°{} marked as done", id);
                    }
                    Err(e) => {
                        // Conversion failed
                        println!("Failed to parse number: {}", e);
                    }
                }
                continue;
            }
            "show" => {
                let todos_to_show = match parts[1] {
                    "done" => todo_repo.show_todos(&ShowTodosOptions::Done),
                    "todo" => todo_repo.show_todos(&ShowTodosOptions::Todo),
                    _ => todo_repo.show_todos(&ShowTodosOptions::All),
                };
                println!("############################################");
                for (index, todo) in todos_to_show.iter().enumerate() {
                    println!(" • {}: {}", index + 1, todo.body());
                }
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
