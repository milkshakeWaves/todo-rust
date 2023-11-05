use chrono::{DateTime, Local};

#[derive(Debug, Clone)]
pub struct Todo {
    id: u32,
    body: String,
    done: bool,
    created_at: DateTime<Local>,
    updated_at: Option<DateTime<Local>>
}

impl Todo {
    pub fn new(id: u32, body: &str) -> Todo {
        Todo {
            id,
            body: body.to_string(), 
            done: false,
            created_at: Local::now(),
            updated_at: None
        }
    }

    pub fn mark_as_done(old_todo: Todo) -> Todo {
        Todo {
            done: true,
            updated_at: Some(Local::now()),
            ..old_todo
        }
    }

    pub fn body(&self) -> &str {
        &self.body
    }

    pub fn is_done(&self) -> bool {
        self.done
    }
}