use tui::{
    backend::{Backend},
    Frame
};

use crate::components::menu;

pub fn first_page<B: Backend>(f: &mut Frame<B>) {
    f.render_widget(menu::menu_render(), f.size());
}
