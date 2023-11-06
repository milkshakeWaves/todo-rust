mod repository;

use crate::repository::ShowTodosOptions;
use repository::{InMemoryRepository, TodoRepository};

use std::io;
use std::io::Write;
use todo_cli::Todo;
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
                handle_create(&mut todo_repo, &parts);
                continue;
            }
            "edit" => {
                println!("edit a todo");
                continue;
            }
            "delete" => {
                match handle_delete(&parts, &mut todo_repo) {
                    Some(todo) => println!("Todo n°{} successfully deleted!", todo.id()),
                    _ => println!("{}", "Cannot delete todo!"),
                }
                continue;
            }
            "done" => {
                match handle_done(&parts, &mut todo_repo) {
                    Some(todo) => println!("Todo n°{} marked as done!", todo.id()),
                    _ => println!("{}", "Cannot mark todo as done!"),
                }
                continue;
            }
            "show" => {
                let todos_to_show: Vec<&Todo> = match parts.get(1) {
                    Some(option) => match *option {
                        "done" => todo_repo.show_todos(&ShowTodosOptions::Done),
                        "todo" => todo_repo.show_todos(&ShowTodosOptions::Todo),
                        _ => todo_repo.show_todos(&ShowTodosOptions::All),
                    },
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

fn handle_done(parts: &Vec<&str>, todo_repo: &mut InMemoryRepository) -> Option<Todo> {
    parts.get(1).and_then(|index| {
        index
            .parse::<u32>()
            .ok()
            .and_then(|id| todo_repo.mark_as_done(id))
    })
}

fn handle_delete(parts: &Vec<&str>, todo_repo: &mut InMemoryRepository) -> Option<Todo> {
    parts.get(1).and_then(|index| {
        index
            .parse::<u32>()
            .ok()
            .and_then(|id| todo_repo.delete_todo(id))
    })
}

fn handle_create(todo_repo: &mut InMemoryRepository, parts: &Vec<&str>) {
    println!("Creating a new todo...");
    let inserted_todo = todo_repo.create_todo(&*parts[1..].join(" "));
    println!("Todo {:?} successfully created!", inserted_todo);
}
