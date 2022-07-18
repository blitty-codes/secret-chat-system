use tui::{
    widgets::{
        Paragraph,
        Block,
        Borders,
        Wrap
    },
    style::{
        Style,
        Color
    }
};

use crate::input::{ChatMessages, InputMode};

pub fn input_box (chat: ChatMessages) -> Paragraph<'static> {
    let input = Paragraph::new(chat.input.clone())
        .style(match chat.mode {
            InputMode::Normal => Style::default(),
            InputMode::Editing => Style::default().fg(Color::Yellow),
        })
        .block(
            Block::default()
                .title("Input")
                .borders(Borders::ALL)
        )
        .wrap(Wrap { trim: true });

    input
}
