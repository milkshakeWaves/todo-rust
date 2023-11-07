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
        let todo = Todo::new(id, todo);
        let todo_clone = todo.clone();
        self.id += 1;
        self.db.insert(id, todo);
        todo_clone
    }

    fn show_todos(&self, options: &ShowTodosOptions) -> Vec<Todo> {
        let values = self.db.values();
        match options {
            ShowTodosOptions::Done => values.filter(|t| t.is_done()).cloned().collect(),
            ShowTodosOptions::Pending => values.filter(|t| !t.is_done()).cloned().collect(),
            _ => values.cloned().collect(),
        }
    }

    fn delete_todo(&mut self, id: u32) -> Option<Todo> {
        self.db.remove(&id)
    }

    fn mark_as_done(&mut self, id: u32) -> Option<Todo> {
        if let Some(todo) = self.db.get_mut(&id) {
            todo.mark_as_done();
            Some(todo.clone())
        } else {
            None
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

#[cfg(test)]
mod tests;
