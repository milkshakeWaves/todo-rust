pub trait TaskRepository {
    // Define functions for interacting with storage: create, edit, delete, list_tasks, etc.
}

// Implement in-memory storage repository
mod in_memory;
pub use in_memory::InMemoryRepository;
