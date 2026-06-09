use ratatui::{
    Frame,
    layout::{Constraint, Layout},
    widgets::{Block, List, ListItem, Paragraph},
};

use crate::app::App;

pub fn render_list(frame: &mut Frame, app_state: &mut App) {
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
