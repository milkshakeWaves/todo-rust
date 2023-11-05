use todo_cli::Todo;

use std::collections::HashMap;

use super::{TodoRepository, ShowTodosOptions};

pub struct InMemoryRepository {
    db: HashMap<u32, Todo>,
    id: u32
}

impl TodoRepository for InMemoryRepository {
    fn create_todo(&mut self, todo: &str) -> Option<Todo> {
        let id: u32 = self.id;
        self.id += 1;
        let todo: Todo = Todo::new(id, todo);
        self.db.insert(id, todo.clone())
    }

    fn show_todos(&self, options: &ShowTodosOptions) -> Vec<&Todo> {
        Vec::from_iter(self.db.values())
    }
}

impl InMemoryRepository {
    pub fn new() -> InMemoryRepository {
        InMemoryRepository { db: HashMap::new(), id: 0 }
    }
}
