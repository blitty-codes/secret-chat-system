use tui::{
    text::{Spans, Span},
    style::{Style, Color},
    widgets::{Block, Tabs, Borders},
    symbols::DOT
};

pub fn menu_render() -> Tabs<'static> {
    // let titles = title_render(t.borrow_mut());

    let titles = ["Chat", "Servers", "Quit"]
        .iter()
        .cloned()
        .map(|string| {
            let (first, rest) = string.split_at(1);
            
            let f = Span::styled(first.to_string(),
                Style::default()
                    .bg(Color::Rgb(102, 0, 102))
                    .fg(Color::Rgb(153, 153, 255))
            );
            let r = Span::from(rest);

            Spans::from(vec![
                f,
                r
            ])
        })
        .collect();

    Tabs::new(titles)
        .block(
            Block::default()
                .title("Menu")
                .borders(Borders::ALL)
        )
        .style(
            Style::default()
                .fg(Color::Cyan)
        )
        .divider(DOT)
}
