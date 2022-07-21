use std::io::Write;
use std::sync::mpsc::{TryRecvError, self};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::{io, thread, fs, env};

use tui::layout::Alignment;
use tui::style::{Style, Color};
use tui::text::Text;
use tui::widgets::{Paragraph, Block, Borders, Wrap};
use tui::{
    backend::CrosstermBackend,
    Terminal,
};

use crossterm::{
    event::{
        DisableMouseCapture,
        EnableMouseCapture,
        read,
        Event,
        KeyCode
    },
    execute,
    terminal::{
        disable_raw_mode,
        enable_raw_mode,
        EnterAlternateScreen,
        LeaveAlternateScreen,
        EnableLineWrap,
    },
    cursor::{
        SetCursorShape,
        CursorShape
    }
};

mod ui;
mod network;
mod config;

use crate::ui::*;
use crate::network::client::Client;

const CONF_FILE: &str = "s-conf.yaml";

// https://docs.rs/tui/latest/tui/
// https://docs.rs/crossterm/latest/crossterm/event/struct.KeyEvent.html
fn main() -> Result<(), io::Error> {
    // search configuration
    let path = env::current_dir()?;
    let path = path.join(CONF_FILE);

    if !path.exists() {
        // create default configuration
        let conf = config::default_configuration();
        let mut file = fs::File::create(CONF_FILE).unwrap();
        file.write_all(conf.as_bytes()).unwrap();
    }

    let conf = fs::read_to_string(path.clone()).unwrap();
    let mut conf = config::read_configuration(conf);
    // TODO: Recoger en servidor y cliente una clave para enviar la clave publica

    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout,
        EnterAlternateScreen,
        EnableMouseCapture,
        EnableLineWrap,
        SetCursorShape(CursorShape::Block)
    )?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    if !conf.has_default_server() {
        loop {
            terminal.draw(|f| {
                let style = Style::default()
                    .fg(Color::LightRed);
                let txt = Text::styled("Default server needed, please set value on configuration file.\n('q' to exit)", style);
                
                let par = Paragraph::new(
                    txt
                )
                    .block(
                        Block::default()
                            .borders(Borders::NONE)
                    )
                    .alignment(Alignment::Center)
                    .wrap(Wrap { trim: true });   

                f.render_widget(par, f.size());
            })?;
            let event = read()?;
            
            match event {
                Event::Key(e) => {
                    if KeyCode::Char('q') == e.code {
                        execute!(
                            terminal.backend_mut(),
                            LeaveAlternateScreen,
                            DisableMouseCapture,
                            SetCursorShape(CursorShape::Line)  
                        )?;
                    
                        terminal.show_cursor()?;
                        disable_raw_mode()?;
                        
                        return Ok(());
                    }
                },
                _ => {}
            }
        }
    }
    
    loop {
        terminal.draw(|f| {
            first_page::first_page(f);
        })?;
        //if crossterm::event::poll(Duration::from_millis(100))? {
        let event = read()?;
            
        match event {
            Event::Key(event) => {
                if KeyCode::Char('c') == event.code {
                    let mut me: Client = Client::new(conf.get_nick(), conf.get_default_server());

                    match me.first_hand_shake() {
                        Ok(_) => (),
                        Err(e) => {
                            panic!("Error: {:?}", e);
                        },
                    }
                    
                    me.send_string_id("Buenos días a todos!!".to_string()).unwrap();                

                    let (tx, rx) = mpsc::channel();

                    let client = Arc::new( Mutex::new(me.clone()));
                    let msg: Arc<Mutex<Vec<(String, String)>>> = Arc::new(Mutex::new(Vec::new()));

                    let c = Arc::clone(&client);
                    let msg_copy = Arc::clone(&msg);
                    thread::spawn(move || loop {
                        match c.try_lock() {
                            Ok(mut f) => {
                                match msg_copy.try_lock() {
                                    Ok(mut m) => {
                                        let res = f.fetch_msg().unwrap();
                                        // println!("res: {:?}", res);
                                        *m = res;
                                        std::mem::drop(m);
                                    },
                                    Err(_) => {
                                        // println!("Inside err: {:?}", err);
                                    }
                                }

                                std::mem::drop(f);
                            },
                            Err(_) => {
                                // println!("Outside err: {:?}", err);
                            },
                        }

                        match rx.try_recv() {
                            Ok(_) | Err(TryRecvError::Disconnected) => {
                                // println!("Terminating.");
                                break;
                            }
                            Err(TryRecvError::Empty) => {}
                        }

                        thread::sleep(Duration::new(2, 0));
                    });
                    chat_page::chat_layout(&mut terminal, &mut me, msg)?;
                    let _ = tx.send(());
                    // recv_msgs.join().unwrap();
                }
                else if KeyCode::Char('s') == event.code {
                    listserver_page::render_servers(&mut terminal, &mut conf)?;
                    config::save_configuration(&conf, path.clone());
                }
                else if KeyCode::Char('q') == event.code {
                    execute!(
                        terminal.backend_mut(),
                        LeaveAlternateScreen,
                        DisableMouseCapture,
                        SetCursorShape(CursorShape::Line)  
                    )?;
                
                    terminal.show_cursor()?;
                    disable_raw_mode()?;

                    break;
                }
            },
            _ => {}
        }
    }
    Ok(())
}
