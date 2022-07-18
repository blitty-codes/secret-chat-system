use tui::{
    widgets::{
        Paragraph,
        Block,
        Borders
    },
    text::{
        Span,
        Spans,
        Text
    },
    style::{
        Style,
        Modifier,
        Color
    },
    layout::Alignment
};

use crate::input::InputMode;

pub fn mode_render (mode: &InputMode) -> Paragraph<'static> {
    let (msg, style) = match mode.clone() {
        InputMode::Normal => (
            vec![
                Span::raw("Press "),
                Span::styled("e", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to start editing."),
            ],
            Style::default().add_modifier(Modifier::RAPID_BLINK),
        ),
        InputMode::Editing => (
            vec![
                Span::raw("Press "),
                Span::styled("Esc", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to stop editing."),
            ],
            Style::default(),
        ),
    };

    let mut text = Text::from(Spans::from(msg));
    text.patch_style(style);
    let help_message = Paragraph::new(text);

    help_message
        .block(
            Block::default()
                .title("Mode")
                .borders(Borders::ALL)
        )
        .alignment(Alignment::Center)
        .style(
            Style::default()
                .fg(Color::Yellow)
        )
}
