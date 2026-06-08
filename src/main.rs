use crossterm::event::KeyCode;
use std::path::PathBuf;

use crossterm::event::Event;
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Layout, Margin},
    widgets::{Block, Clear, List, ListItem, Paragraph},
};
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize, Debug)]
struct Todo {
    title: String,
    description: String,
}

enum Screen {
    List,
    CreateTodo,
}

#[derive(PartialEq)]
enum CreateTodoItemState {
    Title,
    Desc,
}

struct CreateTodoItem {
    title: String,
    description: String,
    state: CreateTodoItemState,
}

struct App {
    todos: Vec<Todo>,
    selected: usize,
    screen: Screen,
    input: CreateTodoItem,
    path: PathBuf,
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

    fn save_file(&mut self) {
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

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let path = PathBuf::from("todos.json");
    let mut app_state = App {
        todos: vec![],
        selected: 0,
        screen: Screen::List,
        input: CreateTodoItem {
            title: String::new(),
            description: String::new(),
            state: CreateTodoItemState::Title,
        },
        path: path,
    };
    app_state.read_file();
    ratatui::run(|terminal| app(terminal, &mut app_state))?;
    Ok(())
}

fn app(terminal: &mut DefaultTerminal, app_state: &mut App) -> std::io::Result<()> {
    loop {
        terminal.draw(|frame| render(frame, app_state))?;
        if let Event::Key(key) = crossterm::event::read()? {
            match app_state.screen {
                Screen::List => match key.code {
                    KeyCode::Char('a') => app_state.screen = Screen::CreateTodo,
                    KeyCode::Char('q') => {
                        app_state.save_file();
                        break Ok(());
                    }
                    KeyCode::Char('j') => app_state.increment_index(),
                    KeyCode::Char('k') => app_state.decrement_index(),
                    _ => {}
                },
                Screen::CreateTodo => match key.code {
                    _ => {
                        app_state.handle_create_todo_item_inputs(key.code);
                    }
                },
            }
        }
    }
}

fn render(frame: &mut Frame, app_state: &mut App) {
    match app_state.screen {
        Screen::List => render_list(frame, app_state),
        Screen::CreateTodo => render_create_todo(frame, app_state),
    }
}

fn render_list(frame: &mut Frame, app_state: &mut App) {
    let chunks = Layout::horizontal([Constraint::Percentage(40), Constraint::Percentage(60)])
        .split(frame.area());

    let items: Vec<ListItem> = app_state
        .todos
        .iter()
        .map(|t| ListItem::new(t.title.clone()))
        .collect();
    let list = List::new(items).block(Block::bordered().title("TODO"));
    frame.render_widget(list, chunks[0]);

    let description = app_state
        .todos
        .get(app_state.selected)
        .map(|t| t.description.as_str())
        .unwrap_or("No todo");

    let desc_block = Paragraph::new(description).block(Block::bordered().title("Description"));

    frame.render_widget(desc_block, chunks[1]);
}
fn render_create_todo(frame: &mut Frame, app_state: &mut App) {
    let popup_bloc = Block::bordered().title("Create TODO");

    let centered_area = frame.area().centered(
        ratatui::layout::Constraint::Percentage(60),
        ratatui::layout::Constraint::Percentage(40),
    );

    frame.render_widget(Clear, centered_area);

    let paragraph = Paragraph::new("test").block(popup_bloc);

    frame.render_widget(paragraph, centered_area);

    let inner = centered_area.inner(Margin::new(1, 1));

    let chunks = Layout::vertical([
        Constraint::Length(3),
        Constraint::Length(3),
        Constraint::Length(1),
    ])
    .split(inner);

    let title_block = Block::bordered().title("Title");
    let title_paragraph = Paragraph::new(app_state.input.title.as_str()).block(title_block);

    frame.render_widget(title_paragraph, chunks[0]);

    let desc_block = Block::bordered().title("Description");
    let desc_paragraph = Paragraph::new(app_state.input.description.as_str()).block(desc_block);

    frame.render_widget(desc_paragraph, chunks[1]);

    let hints = Paragraph::new("Tab: change | Enter: validate | Esc: cancel ");
    frame.render_widget(hints, chunks[2]);
}
