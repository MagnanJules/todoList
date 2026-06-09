use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Todo {
    pub title: String,
    pub description: String,
}

pub enum Screen {
    List,
    CreateTodo,
}

#[derive(PartialEq)]
pub enum CreateTodoItemState {
    Title,
    Desc,
}

pub struct CreateTodoItem {
    pub title: String,
    pub description: String,
    pub state: CreateTodoItemState,
}
