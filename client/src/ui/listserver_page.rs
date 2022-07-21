use std::io;
use crossterm::event::{
        Event,
        read,
        KeyCode
    };
use tui::{
    backend::Backend,
    Frame,
    layout::{Layout, Direction, Constraint, Rect, Alignment},
    widgets::{Tabs, Block, Borders, Clear, Paragraph},
    style::{Color, Style, Modifier},
    symbols::DOT,
    Terminal,
    text::{Span, Spans}
};

use crate::config::Configuration;

pub fn list_server<B: Backend>(f: &mut Frame<B>, conf: &Configuration, selected: usize, popup: bool) {
    let servers: Vec<String> = conf.get_server_list()
        .iter()
        .map(|f| {
            f.to_string()
        }).collect();

    let servers = servers
        .iter()
        .map(|string| {
            let f = Span::styled(string.to_string(),
                Style::default()
                    .fg(Color::Rgb(51, 204, 255))
            );

            Spans::from(vec![
                f
            ])
        })
        .collect();

    let tabs = Tabs::new(servers)
        .block(
            Block::default()
                .title("Server list")
                .borders(Borders::ALL)
        )
        .style(
            Style::default()
                .fg(Color::Cyan)
        )
        .select(selected)
        .highlight_style(
            Style::default()
                .fg(Color::Rgb(153, 153, 255))
                .add_modifier(Modifier::BOLD)
        )
        .divider(DOT);

    let size = f.size();
    if popup {
        let top_block = Block::default()
            .title(format!("Do you want server {} as default?", conf.get_server_list()[selected]))
            .title_alignment(Alignment::Center)
            .style(
                Style::default()
                    .fg(Color::Cyan)
            )
            .borders(Borders::TOP | Borders::LEFT | Borders::RIGHT);

        let bottom_block = Block::default()
            .style(
                Style::default()
                    .fg(Color::Cyan)
            )
            .borders(Borders::BOTTOM | Borders::LEFT | Borders::RIGHT);


        let titles = ["Yes", "No"]
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
            
        let area = centered_rect(60, 20, size);
        let layout = Layout::default()
            .constraints([
                Constraint::Percentage(50),
                Constraint::Percentage(50)
            ].as_ref())
            .split(area);

        let current_server = Paragraph::new(
            format!("Current server: {:?}", conf.get_default_server().unwrap())
        )
            .block(top_block)
            .alignment(Alignment::Center)
            .style(
                Style::default()
                    .fg(Color::Yellow)
            );
    
        let options = Tabs::new(titles)
            .block(bottom_block)
            .style(
                Style::default()
                    .fg(Color::Cyan)
            );

        f.render_widget(Clear, area); //this clears out the background
        f.render_widget(current_server, layout[0]);
        f.render_widget(options, layout[1]);
    } else {
        f.render_widget(tabs, size);
    }
}

/// helper function to create a centered rect using up certain percentage of the available rect `r`
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage((100 - percent_y) / 2),
                Constraint::Percentage(percent_y),
                Constraint::Percentage((100 - percent_y) / 2),
            ]
            .as_ref(),
        )
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage((100 - percent_x) / 2),
                Constraint::Percentage(percent_x),
                Constraint::Percentage((100 - percent_x) / 2),
            ]
            .as_ref(),
        )
        .split(popup_layout[1])[1]
}

pub fn render_servers<B: Backend>(terminal: &mut Terminal<B>, conf: &mut Configuration) -> Result<(), io::Error> {
    let mut selected = 0;
    let mut popup = false;
    loop {
        terminal.draw(|f| list_server(f, conf, selected, popup))?;
        let event = read()?;

        match event {
            Event::Key(event) => {
                if !popup {
                    if KeyCode::Char('q') == event.code {
                        return Ok(());
                    } else if KeyCode::Right == event.code && selected < conf.server_list_len()-1  {
                        selected+=1;
                    } else if KeyCode::Left == event.code && selected > 0 {
                        selected-=1;
                    } else if KeyCode::Enter == event.code {
                        popup = true;
                    }    
                } else {
                    if KeyCode::Char('n') == event.code {
                        popup = false;
                    } else if KeyCode::Char('y') == event.code {
                        conf.set_default_server(selected);
                        popup = false
                    }
                }
            }
            _ => {}
        }
    }


}
