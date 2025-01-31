use std::time::{Duration, Instant};

use crate::editor::{size::Size, terminal::Terminal};

use super::uicomponent::UIComponent;

const DEFAULT_DURATION: Duration = Duration::from_secs(5);

struct Message {
    text: String,
    time: Instant,
}

impl Default for Message {
    fn default() -> Self {
        Self {
            text: String::new(),
            time: Instant::now(),
        }
    }
}

impl Message {
    fn new(message: &str) -> Self {
        Self {
            text: message.to_string(),
            time: Instant::now(),
        }
    }

    fn is_expired(&self) -> bool {
        Instant::now().duration_since(self.time) > DEFAULT_DURATION
    }
}

#[derive(Default)]
pub struct MessageBar {
    current_message: Message,
    needs_redraw: bool,
    cleared_after_expiry: bool,
}

impl MessageBar {
    pub fn update_message(&mut self, new_message: String) {
        self.current_message = Message::new(&new_message);
        self.cleared_after_expiry = false;
        self.set_needs_redraw(true);
    }
}

impl UIComponent for MessageBar {
    fn set_needs_redraw(&mut self, value: bool) {
        self.needs_redraw = value;
    }

    fn needs_redraw(&self) -> bool {
        (!self.cleared_after_expiry && self.current_message.is_expired()) || self.needs_redraw
    }

    fn set_size(&mut self, _: Size) {}

    fn draw(&mut self, origin_y: usize) -> Result<(), std::io::Error> {
        if self.current_message.is_expired() {
            self.cleared_after_expiry = true;
        }
        let message = if self.current_message.is_expired() {
            None
        } else {
            Some(self.current_message.text.clone())
        };

        Terminal::print_row(origin_y, &message.unwrap_or_default())
    }
}
