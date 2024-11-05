use serde::Serialize;

#[derive(Serialize)]
pub struct Todo {
    id: i32,
    title: String,
    completed: bool,
}

impl Todo {
    pub fn new(id: i32, title: String, completed: bool) -> Self {
        Self {
            id,
            title,
            completed,
        }
    }
}
