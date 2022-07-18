#[derive(Clone)]
pub struct ScrollControll {
    pub offset: u16,
    pub area_height: u16,

    // auto scroll, activated when no scrolling, deactivated when scrolling
    pub is_auto_offset: bool
}

impl Default for ScrollControll {
    fn default() -> ScrollControll {
        ScrollControll {
            offset: 0,
            area_height: 0,
            is_auto_offset: true,
        }
    }
}

#[derive(Clone)]
pub struct ChatMessages {
    // current value of the input
    pub input: String,

    // history messages
    pub messages: Vec<(String, String)>,

    // input mode
    pub mode: InputMode
}

impl Default for ChatMessages {
    fn default() -> ChatMessages {
        ChatMessages {
            input: String::new(),
            messages: Vec::new(),
            mode: InputMode::Normal,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum InputMode {
    Editing,
    Normal
}
