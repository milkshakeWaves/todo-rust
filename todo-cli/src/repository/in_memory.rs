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

    fn show_todos(&self, options: &ShowTodosOptions) -> Box<dyn Iterator<Item = &Todo>> {
        let values = self.db.values();
        match options {
            ShowTodosOptions::Done => Box::new(values.filter(|t| t.is_done())),
            ShowTodosOptions::Todo => Box::new(values.filter(|t| !t.is_done())),
            _ => Box::new(values.into_iter()),
        }
    }

    fn delete_todo(&mut self, id: u32) -> Option<Todo> {
        self.db.remove(&id)
    }

    fn mark_as_done(&mut self, id: u32) -> bool {
        if let Some(todo) = self.db.get_mut(&id) {
            todo.mark_as_done();
            true
        } else {
            false
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
