mod create;
mod list;

use crate::app::App;
use crate::types::Screen;
use ratatui::Frame;

pub fn render(frame: &mut Frame, app_state: &mut App) {
    match app_state.screen {
        Screen::List => list::render_list(frame, app_state),
        Screen::CreateTodo => create::render_create_todo(frame, app_state),
    }
}
