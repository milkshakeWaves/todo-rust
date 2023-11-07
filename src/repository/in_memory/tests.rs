#[cfg(test)]
mod tests {
    use todo_cli::Todo;

    use crate::repository::{InMemoryRepository, TodoRepository, ShowTodosOptions};

    #[test]
    fn create_todo_test() {
        // Arrange
        let mut in_memory_db = InMemoryRepository::new();
        let todo_description = "This is a mock todo";

        // Act
        let inserted_todo = in_memory_db.create_todo(todo_description);

        // Assert
        assert_eq!(inserted_todo.body(), todo_description);
        assert_eq!(inserted_todo.id(), 0);
        assert!(in_memory_db.db.get(&0).is_some());
    }

    #[test]
    fn delete_todo_test() {
        // Arrange
        let mut in_memory_db = InMemoryRepository::new();
        let todo_description = "This is a mock todo";

        // Todo 0 does not exist yet
        assert!(in_memory_db.delete_todo(0).is_none());
        // Todo 10 does not exist either
        assert!(in_memory_db.delete_todo(10).is_none());

        add_todo(&mut in_memory_db, todo_description);

        // Act
        let deleted_todo = in_memory_db.delete_todo(0);

        // Assert
        assert!(deleted_todo.is_some());
        assert!(in_memory_db.db.get(&0).is_none());
        // Todo 0 has already been deleted
        assert!(in_memory_db.delete_todo(0).is_none());
    }

    #[test]
    fn mark_as_done_test() {
        // Arrange
        let mut in_memory_db = InMemoryRepository::new();
        for desc in ["First", "Second", "Third", "Fourth"] {
            add_todo(&mut in_memory_db, desc);
        }

        // Act
        let done_todo = in_memory_db.mark_as_done(2);

        // Assert
        assert!(done_todo.is_some_and(|t| t.is_done()));
        assert!(in_memory_db.db.get(&0).is_some_and(|t| !t.is_done()));
        assert!(in_memory_db.db.get(&1).is_some_and(|t| !t.is_done()));
        assert!(in_memory_db.db.get(&2).is_some_and(|t| t.is_done()));
        assert!(in_memory_db.db.get(&3).is_some_and(|t| !t.is_done()));
    }

    #[test]
    fn show_all_todos_test() {
        // Arrange
        let mut in_memory_db = InMemoryRepository::new();
        let todos_to_insert = ["First", "Second", "Third", "Fourth"];
        for desc in todos_to_insert {
            add_todo(&mut in_memory_db, desc);
        }

        // Act
        in_memory_db.mark_as_done(2);
        let todos = in_memory_db.show_todos(&ShowTodosOptions::All);

        // Assert
        assert_eq!(todos.len(), todos_to_insert.len());
    }

    #[test]
    fn show_done_todos_test() {
        // Arrange
        let mut in_memory_db = InMemoryRepository::new();
        let todos_to_insert = ["First", "Second", "Third", "Fourth"];
        for desc in todos_to_insert {
            add_todo(&mut in_memory_db, desc);
        }

        // Act
        in_memory_db.mark_as_done(2);
        let todos = in_memory_db.show_todos(&ShowTodosOptions::Done);

        // Assert
        assert_eq!(todos.len(), 1);
        assert!(todos.get(0).is_some_and(|t| t.is_done() && t.id() == 2));
    }

    #[test]
    fn show_pending_todos_test() {
        // Arrange
        let mut in_memory_db = InMemoryRepository::new();
        let todos_to_insert = ["First", "Second", "Third", "Fourth"];
        for desc in todos_to_insert {
            add_todo(&mut in_memory_db, desc);
        }

        // Act
        in_memory_db.mark_as_done(2);
        let todos = in_memory_db.show_todos(&ShowTodosOptions::Pending);

        // Assert
        assert_eq!(todos.len(), 3);
        for todo in todos {
            assert!(!todo.is_done())
        }
    }

    fn add_todo(db: &mut InMemoryRepository, todo: &str) -> Todo {
        db.create_todo(todo)
    }
}
