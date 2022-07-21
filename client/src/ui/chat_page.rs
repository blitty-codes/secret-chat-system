use std::{time::Duration, sync::{Arc, Mutex}};
use crossterm::event::{
    self,
    Event,
    KeyCode
};
use tui::{
    backend::Backend,
    layout::{
        Layout,
        Constraint,
        Direction
    },
    Frame,
    Terminal
};

use crate::network::client::Client;
use crate::input::*;
use crate::components::{
    menu,
    mode,
    chat,
    input
};

#[derive(Clone)]
pub struct Configuration {
    pub chat: ChatMessages,
    pub scroll: ScrollControll
}

impl Default for Configuration {
    fn default() -> Configuration {
        Configuration {
            chat: ChatMessages::default(),
            scroll: ScrollControll::default()
        }
    }
}

pub fn ui<B: Backend>(f: &mut Frame<B>, conf: &mut Configuration) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .horizontal_margin(1)
        .constraints(
            [
                Constraint::Percentage(12),
                Constraint::Percentage(80),
                Constraint::Percentage(10)
            ].as_ref()
        )
        .split(f.size());

    conf.scroll.area_height = chunks[2].y - chunks[2].height - 2;

    let top = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage(70),
                Constraint::Percentage(30)
            ].as_ref()
        )
        .split(chunks[0]);

    let menu = menu::menu_render();
    f.render_widget(menu, top[0]);

    let editing_mode = mode::mode_render(&conf.chat.mode);
    f.render_widget(editing_mode, top[1]);

    let chat = chat::chat_render(conf.clone());
    f.render_widget(chat, chunks[1]);

    let input = input::input_box(conf.chat.clone());
    f.render_widget(input, chunks[2]);

    match conf.chat.mode {
        InputMode::Normal => {},
        InputMode::Editing => {
            f.set_cursor(
                chunks[2].x + conf.chat.input.len() as u16 + 1,
                chunks[2].y + 1
            )
        },
    }
}

// https://github.com/fdehau/tui-rs/blob/master/examples/user_input.rs
pub fn chat_layout<B: Backend> (terminal: &mut Terminal<B>, me: &mut Client, msg: Arc<Mutex<Vec<(String, String)>>>) -> Result<(), std::io::Error> {
    let conf = &mut Configuration::default();

    loop {
        terminal.draw(|f| ui(f, conf))?;

        if conf.chat.mode == InputMode::Normal {
            conf.scroll.is_auto_offset = false;
        }

        if conf.chat.mode == InputMode::Editing {
            conf.scroll.is_auto_offset = true;
        }

        match msg.try_lock() {
            Ok(mut m) => {
                conf.chat.messages.append(&mut (*m.clone()).to_vec());
                *m = Vec::new();
                std::mem::drop(m);
            },
            // TODO: logger
            Err(_) => {},
        }

        if event::poll(Duration::from_millis(100))? {
            match event::read()? {
                Event::Key(key) => {
                    match conf.chat.mode {
                        InputMode::Normal => match key.code {
                                KeyCode::Char('e') => {
                                    conf.chat.mode = InputMode::Editing;
                                }
                                KeyCode::Char('q') => {
                                    return Ok(());
                                }
                                _ => {}
                            },
                        InputMode::Editing => {
                            match key.code {
                                KeyCode::Enter => {
                                    let msg: String = conf.chat.input.drain(..).collect();
                                    conf.chat.messages.push(
                                        (me.get_name().to_string(), msg.clone())
                                    );

                                    me.send_string_id(msg).unwrap();
                                }
                                KeyCode::Char(c) => {
                                    conf.chat.input.push(c);
                                }
                                KeyCode::Backspace => {
                                    conf.chat.input.pop();
                                }
                                KeyCode::Esc => {
                                    conf.chat.mode = InputMode::Normal;
                                }
                                _ => {}
                            }
                        }
                    }
                },
                Event::Mouse(event) => {
                    let mode = conf.chat.mode.clone();

                    if mode == InputMode::Normal {
                        if event.kind == event::MouseEventKind::ScrollUp && conf.scroll.offset > 0 {
                            conf.scroll.offset -= 1;
                        } else if event.kind == event::MouseEventKind::ScrollDown && (conf.scroll.offset as i16) < ((conf.chat.messages.len() as i16) - (conf.scroll.area_height as i16)) {
                            conf.scroll.offset += 1;
                        }
                    }
                },
                _ => {},
            }
        }
    }
}
