use crate::types::{CreateTodoItem, CreateTodoItemState, Screen, Todo};

use crossterm::event::KeyCode;
use std::fs;
use std::path::PathBuf;

pub struct App {
    pub todos: Vec<Todo>,
    pub selected: usize,
    pub screen: Screen,
    pub input: CreateTodoItem,
    pub path: PathBuf,
}

impl App {
    pub fn handle_create_todo_item_inputs(&mut self, key: KeyCode) {
        match key {
            KeyCode::Tab => {
                if self.input.state == CreateTodoItemState::Title {
                    self.input.state = CreateTodoItemState::Desc;
                } else {
                    self.input.state = CreateTodoItemState::Title;
                }
            }
            KeyCode::Char(c) => {
                if self.input.state == CreateTodoItemState::Title {
                    self.input.title.push(c);
                } else {
                    self.input.description.push(c);
                }
            }
            KeyCode::Backspace => {
                if self.input.state == CreateTodoItemState::Title {
                    self.input.title.pop();
                } else {
                    self.input.description.pop();
                }
            }
            KeyCode::Enter => {
                self.todos.push(Todo {
                    title: std::mem::take(&mut self.input.title),
                    description: std::mem::take(&mut self.input.description),
                });
            }
            KeyCode::Esc => {
                self.screen = Screen::List;
            }
            _ => {}
        }
    }

    pub fn save_file(&mut self) {
        if !self.path.exists() {
            panic!("The file has never been created.");
        }
        let content = serde_json::to_string_pretty(&self.todos).unwrap();
        fs::write(&mut self.path, content).unwrap();
    }

    pub fn read_file(&mut self) {
        if !self.path.exists() {
            fs::write(&self.path, "[]").unwrap();
        }
        let content = fs::read_to_string(&mut self.path).unwrap();
        let todo_list: Vec<Todo> = serde_json::from_str(&content).unwrap();
        self.todos = todo_list;
    }

    pub fn increment_index(&mut self) {
        if self.selected >= self.todos.len() - 1 {
            return;
        }
        self.selected += 1;
    }

    pub fn decrement_index(&mut self) {
        if self.selected <= 0 {
            return;
        }
        self.selected -= 1;
    }
}
