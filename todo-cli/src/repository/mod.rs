use todo_cli::Todo;

pub trait TodoRepository {
    // Define functions for interacting with storage: create, edit, delete, list_tasks, etc.
    fn create_todo(&mut self, todo: &str) -> Option<Todo>;
    fn show_todos(&self, options: &ShowTodosOptions) -> Vec<&Todo>;
}

pub enum ShowTodosOptions {
    All,
    Done,
    Todo
}

// Implement in-memory storage repository
mod in_memory;
pub use in_memory::InMemoryRepository;

