use tui::{
    text::{Spans, Span},
    style::{Style, Color},
    widgets::{Block, Tabs, Borders},
    symbols::DOT
};

fn title_render () -> Vec<Spans<'static>> {
    ["Chat", "Add Server", "Servers", "Quit"]
        .iter()
        .cloned()
        .map(|t| {
            let (first, rest) = t.split_at(1);
            
            let f = Span::styled(first,
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
        .collect()
}

pub fn menu_render () -> Tabs<'static> {
    let titles = title_render();

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
        .highlight_style(
            Style::default()
                .fg(Color::Rgb(51, 204, 255))
        )
        .divider(DOT)
}
