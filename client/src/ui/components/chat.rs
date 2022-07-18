use tui::{
    text::{
        Spans,
        Span,
        Text
    },
    widgets::{
        Paragraph,
        Block,
        Borders,
        Wrap
    }
};

use crate::chat_page::Configuration;

fn get_span (messages: Vec<(String, String)>) -> Vec<Spans<'static>> {
    messages
        .iter()
        .map(|m| {
            Spans::from(
                Span::raw(format!("{}: {}", m.0, m.1))
            )
        })
        .collect()
}

pub fn chat_render (conf: Configuration) -> Paragraph<'static> {
    let chat = conf.chat;
    let mut scroll = conf.scroll;
    let length = chat.messages.len();
    let messages: Vec<Spans>;
    
    if length >= scroll.area_height as usize {
        let max: usize;
        let min: usize;
        let msg: Vec<(String, String)>;
        
        if scroll.is_auto_offset {
            min = length - scroll.area_height as usize;
            max = length;
        } else {
            if length >= scroll.area_height as usize {
                min = length - (scroll.area_height + scroll.offset) as usize;
                max = (scroll.area_height as usize) + min;
            } else {
                min = 0;
                max = scroll.area_height as usize;
            }
        }

        // println!("min: {} - max: {} - len: {} - offset: {} - height: {}", min, max, chat.messages.len(), scroll.offset as usize, scroll.area_height as usize);
        msg = chat.messages[min..max].to_vec();
        messages = get_span(msg);
    } else {
        scroll.offset = 0;

        messages = get_span(chat.messages);
    }

    Paragraph::new(Text::from(messages))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Messages")
        )
        .wrap(Wrap { trim: true })
}
