use todo_cli::Todo;

use std::collections::HashMap;

use super::{ShowTodosOptions, TodoRepository};

pub struct InMemoryRepository {
    db: HashMap<u32, Todo>,
    id: u32,
}

impl TodoRepository for InMemoryRepository {
    fn create_todo(&mut self, todo: &str) -> Todo {
        let id: u32 = self.id;
        let todo: Todo = Todo::new(id, todo);
        self.id += 1;
        self.db.insert(id, todo.clone());
        todo
    }

    fn show_todos(&self, options: &ShowTodosOptions) -> Vec<&Todo> {
        let values = self.db.values();
        match options {
            ShowTodosOptions::Done => Vec::from_iter(values.filter(|t| t.is_done())),
            ShowTodosOptions::Todo => Vec::from_iter(values.filter(|t| !t.is_done())),
            _ => Vec::from_iter(values),
        }
    }

    fn delete_todo(&mut self, id: u32) -> Option<Todo> {
        self.db.remove(&id)
    }

    fn mark_as_done(&mut self, id: u32) -> bool {
        match self.db.get(&id) {
            Some(value) => {
                self.db.insert(id, Todo::mark_as_done(value.clone()));
                true
            }
            _ => false,
        }
    }
}

impl InMemoryRepository {
    pub fn new() -> InMemoryRepository {
        InMemoryRepository {
            db: HashMap::new(),
            id: 0,
        }
    }
}
