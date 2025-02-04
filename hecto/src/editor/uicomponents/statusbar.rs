use std::fmt::format;

use crate::editor::{documentstatus::DocumentStatus, size::Size, terminal::Terminal};

use super::uicomponent::UIComponent;

#[derive(Default)]
pub struct StatusBar {
    current_status: DocumentStatus,
    needs_redraw: bool,
    size: Size,
}

impl StatusBar {
    pub fn update_status(&mut self, new_status: DocumentStatus) {
        if new_status != self.current_status {
            self.current_status = new_status;
            self.set_needs_redraw(true);
        }
    }
}

impl UIComponent for StatusBar {
    fn set_needs_redraw(&mut self, value: bool) {
        self.needs_redraw = value;
    }

    fn needs_redraw(&self) -> bool {
        self.needs_redraw
    }

    fn set_size(&mut self, size: Size) {
        self.size = size;
    }

    fn draw(&mut self, origin_row: usize) -> Result<(), std::io::Error> {
        let line_count = self.current_status.line_count_to_string();
        let modified_indicator = self.current_status.modified_indicator_to_string();
        let beginning = format!(
            "{} - {line_count} {modified_indicator}",
            self.current_status.file_name,
        );

        let position_indicator = self.current_status.position_indicator_to_string();
        let file_type = self.current_status.file_type_to_string();
        let back_part = format!("{file_type} | {position_indicator}");
        let remainder_len = self.size.width.saturating_sub(beginning.len());
        let status = format!("{beginning}{back_part:>remainder_len$}");
        let to_print = if status.len() <= self.size.width {
            Some(status)
        } else {
            None
        };
        Terminal::print_inverted_row(origin_row, to_print.as_deref().unwrap_or_default())
            .expect("Failed to render status bar");
        Ok(())
    }
}
