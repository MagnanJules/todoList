use crossterm::event::KeyCode;
use std::path::PathBuf;
mod app;
mod types;
mod ui;

use app::App;
use types::{CreateTodoItem, CreateTodoItemState, Screen};
use ui::render;

use crossterm::event::Event;
use ratatui::DefaultTerminal;

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
