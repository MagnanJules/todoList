use crossterm::event::KeyCode;
use std::path::PathBuf;

use crossterm::event::Event;
use ratatui::{
    DefaultTerminal, Frame,
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph},
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

struct App {
    todos: Vec<Todo>,
    selected: usize,
    screen: Screen,
}

fn read_file(path: PathBuf) -> Vec<Todo> {
    if !path.exists() {
        fs::write(&path, "[]").unwrap();
    }
    let content = fs::read_to_string(path).unwrap();
    let todo_list: Vec<Todo> = serde_json::from_str(&content).unwrap();
    return todo_list;
}

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let todos = read_file(PathBuf::from("todos.json"));
    let mut app_state = App {
        todos: todos,
        selected: 0,
        screen: Screen::List,
    };
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
                    _ => {}
                },
                Screen::CreateTodo => match key.code {
                    _ => {}
                },
                _ => {}
            }
            match key.code {
                KeyCode::Char('q') => break Ok(()),
                _ => {}
            }
        }
    }
}

fn render(frame: &mut Frame, app_state: &mut App) {
    match app_state.screen {
        Screen::List => render_list(frame, app_state),
        Screen::CreateTodo => render_create_todo(frame, app_state),
        _ => {}
    }
}

fn render_list(frame: &mut Frame, app_state: &mut App) {
    let items: Vec<ListItem> = app_state
        .todos
        .iter()
        .map(|t| ListItem::new(t.title.clone()))
        .collect();
    let list = List::new(items).block(Block::default().borders(Borders::all()).title("TODO"));
    frame.render_widget(list, frame.area());
}
fn render_create_todo(frame: &mut Frame, app_state: &mut App) {
    let popup_bloc = Block::bordered().title("Create TODO");

    let centered_area = frame.area().centered(
        ratatui::layout::Constraint::Percentage(60),
        ratatui::layout::Constraint::Percentage(20),
    );

    frame.render_widget(Clear, centered_area);

    let paragraph = Paragraph::new("test").block(popup_bloc);

    frame.render_widget(paragraph, centered_area);
}
