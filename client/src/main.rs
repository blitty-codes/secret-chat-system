use std::sync::mpsc::{TryRecvError, self};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::{io, thread};
use std::net::{ Ipv4Addr, SocketAddr, SocketAddrV4 };

use rand::Rng;
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
use ui::input::ChatMessages;

mod ui;
mod network;

use crate::ui::*;
use crate::network::client::{Client, Connection};

const PORT : u16 = 8081;
const ADDR : Ipv4Addr = Ipv4Addr::new(127, 0, 0, 1); 

// https://docs.rs/tui/latest/tui/
// https://docs.rs/crossterm/latest/crossterm/event/struct.KeyEvent.html
fn main() -> Result<(), io::Error> {
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

    let random = rand::random::<u8>();
    let mut me: Client = Client::new(format!("rusty child {}", random), None);

    // TODO: check if there are errors
    me.first_hand_shake();
    
    me.send_string_id("Buenos días a todos!!".to_string());

    loop {
        terminal.draw(|f| {
            first_page::first_page(f);
        })?;
        //if crossterm::event::poll(Duration::from_millis(100))? {
        let event = read()?;
            
        match event {
            Event::Key(event) => {
                if KeyCode::Char('c') == event.code {
                    let (tx, rx) = mpsc::channel();

                    let client = Arc::new( Mutex::new(me.clone()));
                    let msg: Arc<Mutex<Vec<(String, String)>>> = Arc::new(Mutex::new(Vec::new()));

                    let c = Arc::clone(&client);
                    let msg_copy = Arc::clone(&msg);
                    let recv_msgs = thread::spawn(move || loop {
                        match c.try_lock() {
                            Ok(mut f) => {
                                match msg_copy.try_lock() {
                                    Ok(mut m) => {
                                        let res = f.fetch_msg().unwrap();
                                        // println!("res: {:?}", res);
                                        *m = res;
                                        std::mem::drop(m);
                                    },
                                    Err(err) => {
                                        // println!("Inside err: {:?}", err);
                                    }
                                }

                                std::mem::drop(f);
                            },
                            Err(err) => {
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
                else if KeyCode::Char('n') == event.code {}
                else if KeyCode::Char('s') == event.code {}
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
