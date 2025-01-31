use std::cmp::min;

use crate::editor::{command::Edit, line::Line, size::Size, terminal::Terminal};

use super::uicomponent::UIComponent;


#[derive(Default)]
pub struct CommandBar {
    prompt: String,
    value: Line,
    needs_redraw: bool,
    size: Size,
}

impl CommandBar {
    pub fn handle_edit_command(&mut self, command: Edit) {
        match command {
            Edit::Insert(c) => self.value.append_char(c),
            Edit::Delete | Edit::InsertNewline => {}
            Edit::DeleteBackward => self.value.delete_last(),
        }
        self.set_needs_redraw(true);
    }

    pub fn caret_position_col(&self) -> usize {
        let max_value = self
            .prompt
            .len()
            .saturating_add(self.value.grapheme_count());
        min(max_value, self.size.width)
    }

    pub fn value(&self) -> String {
        self.value.to_string()
    }

    pub fn set_prompt(&mut self, prompt: &str) {
        self.prompt = prompt.to_string();
        self.set_needs_redraw(true);
    }

    pub fn clean_value(&mut self) {
        self.value = Line::default();
        self.set_needs_redraw(true);
    }
}

impl UIComponent for CommandBar {
    fn set_needs_redraw(&mut self, value: bool) {
        self.needs_redraw = value;
    }

    fn needs_redraw(&self) -> bool {
        self.needs_redraw
    }

    fn set_size(&mut self, size: Size) {
        self.size = size;
    }

    fn draw(&mut self, origin_y: usize) -> Result<(), std::io::Error> {
        let area_for_value = self.size.width.saturating_sub(self.prompt.len());
        let value_end = self.value.width();
        let value_start = value_end.saturating_sub(area_for_value);

        let message = format!("{}{}", self.prompt, self.value.get_visible_graphemes(value_start..value_end));
        let to_print = if message.len() <= self.size.width {
            Some(message)
        } else {
            None
        };
        Terminal::print_row(origin_y, to_print.as_deref().unwrap_or_default())
    }
}
