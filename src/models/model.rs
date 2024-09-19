
pub struct Todo {
    pub(crate) todo_id: u32,
    pub(crate) todo_name: String
}

impl Todo {
    pub fn new(todo_id: u32, todo_name: String) -> Self {
        Todo {
            todo_id,
            todo_name,
        }
    }
}