use tui::{
    backend::Backend,
    Frame, layout::{Layout, Direction, Constraint}
};

use crate::components::{ banner, menu };

pub fn first_page<B: Backend>(f: &mut Frame<B>) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .horizontal_margin(1)
        .constraints(
            [
                Constraint::Percentage(40),
                Constraint::Percentage(60),
            ].as_ref()
        )
        .split(f.size());
    
    f.render_widget(banner::banner(), chunks[0]);
    f.render_widget(menu::menu_render(), chunks[1]);
}
