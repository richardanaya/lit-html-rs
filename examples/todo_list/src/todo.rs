use serde::{Deserialize, Serialize};
use web::*;

#[derive(Serialize, Deserialize)]
pub struct Todo {
    pub completed: bool,
    pub text: String,
}

#[derive(Serialize, Deserialize)]
pub struct TodoList {
    pub items: Vec<Todo>,
}

impl TodoList {
    pub fn save(&self) {
        match serde_json::to_string(self) {
            Ok(s) => local_storage_set_item("todos", &s),
            Err(_) => console_error("error serializing todos"),
        };
    }
    pub fn load() -> Option<TodoList> {
        return match local_storage_get_item("todos") {
            Some(s) => match serde_json::from_str(&s) {
                Ok(s) => Some(s),
                Err(_) => {
                    console_error("error parsing todos");
                    None
                }
            },
            None => None,
        };
    }
}
impl Default for TodoList {
    fn default() -> Self {
        match TodoList::load() {
            Some(tl) => tl,
            None => TodoList {
                items: vec![]
            }
        }
    }
}
