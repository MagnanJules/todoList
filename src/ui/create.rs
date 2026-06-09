use crate::app::App;

use ratatui::{
    Frame,
    layout::{Constraint, Layout, Margin},
    widgets::{Block, Clear, Paragraph},
};

pub fn render_create_todo(frame: &mut Frame, app_state: &mut App) {
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
