use todo_cli::Todo;

pub trait TodoRepository {
    // Define functions for interacting with storage: create, edit, delete, list_tasks, etc.
    fn create_todo(&mut self, todo: &str) -> Todo;
    fn show_todos(&self, options: &ShowTodosOptions) -> Vec<Todo>;
    fn delete_todo(&mut self, id: u32) -> Option<Todo>;
    fn mark_as_done(&mut self, id: u32) -> Option<Todo>;
}

pub enum ShowTodosOptions {
    All,
    Done,
    Pending
}

// Implement in-memory storage repository
mod in_memory;
pub use in_memory::InMemoryRepository;

