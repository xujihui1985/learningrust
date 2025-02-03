use command::{Command, Edit, Move, System};
use crossterm::event::{read, Event, KeyEvent, KeyEventKind};
use std::env;
use std::io::Error as IoError;
use std::panic::{set_hook, take_hook};
mod annotatedstring;
pub mod annotationtype;
mod annotation;

pub(crate) mod command;
pub(crate) mod documentstatus;
pub(crate) mod line;
pub(crate) mod position;
pub(crate) mod size;
pub(crate) mod terminal;
mod uicomponents;
use crate::editor::Command::*;
use crate::editor::Edit::*;
use crate::editor::System::*;
use uicomponents::UIComponent;
pub use annotationtype::AnnotationType;
use annotation::Annotation;
use annotatedstring::AnnotatedString;

use position::Position;
use size::Size;
use terminal::Terminal;
use uicomponents::{CommandBar, MessageBar, StatusBar, View};
// use command::{System, Command};

pub const NAME: &str = env!("CARGO_PKG_NAME");
const HELP_MESSAGE: &str = "Help: Ctrl-S = save | Ctrl-Q = quit";
const QUIT_TIMES: u8 = 3;

#[derive(Debug, PartialEq, Default)]
enum PromptType {
    Search,
    Save,
    #[default]
    None,
}

impl PromptType {
    fn is_none(&self) -> bool {
        matches!(self, Self::None)
    }
}

type Result<T> = std::result::Result<T, IoError>;

#[derive(Default)]
pub struct Editor {
    should_quit: bool,
    view: View,
    status_bar: StatusBar,
    message_bar: MessageBar,
    command_bar: CommandBar,
    prompt_type: PromptType,
    terminal_size: Size,
    title: String,
    quit_times: u8,
}

impl Editor {
    pub fn new() -> Result<Self> {
        let current_hook = take_hook();
        set_hook(Box::new(move |panic_info| {
            let _ = Terminal::terminate();
            current_hook(panic_info);
        }));
        Terminal::initialize()?;
        let mut editor = Self::default();
        let size = Terminal::size().unwrap_or_default();
        editor.handle_resize_command(size);
        editor.update_message(HELP_MESSAGE.to_string());

        let args = env::args().collect::<Vec<_>>();
        if let Some(file_name) = args.get(1) {
            if editor.view.load(file_name).is_err() {
                editor.update_message(format!("Could not open file: {}", file_name));
            }
        }
        editor.refresh_status();
        Ok(editor)
    }

    fn refresh_screen(&mut self) {
        if self.terminal_size.height == 0 || self.terminal_size.width == 0 {
            return;
        }
        let bottom_bar_row = self.terminal_size.height.saturating_sub(1);
        Terminal::hide_caret().expect("failed to hide caret");
        if self.in_prompt() {
            self.command_bar.render(bottom_bar_row);
        } else {
            self.message_bar.render(bottom_bar_row);
        }
        if self.terminal_size.height > 1 {
            self.status_bar
                .render(self.terminal_size.height.saturating_sub(2));
        }
        if self.terminal_size.height > 2 {
            self.view.render(0);
        }
        let new_caret_pos = if self.in_prompt() {
            Position {
                row: bottom_bar_row,
                col: self.command_bar.caret_position_col(),
            }
        } else {
            self.view.caret_position()
        };
        debug_assert!(new_caret_pos.col <= self.terminal_size.width);
        debug_assert!(new_caret_pos.row <= self.terminal_size.height);
        Terminal::move_caret_to(new_caret_pos).expect("failed to move caret");
        Terminal::show_caret().expect("failed to show caret");
        Terminal::execute().expect("failed to execute");
    }

    fn refresh_status(&mut self) {
        let status = self.view.get_status();
        let title = format!("{} - {NAME}", status.file_name,);
        self.status_bar.update_status(status);
        if title != self.title && Terminal::set_title(&title).is_ok() {
            self.title = title;
        }
    }

    fn process_command(&mut self, command: Command) {
        if let System(Resize(size)) = command {
            self.handle_resize_command(size);
            return;
        }

        match self.prompt_type {
            PromptType::Search => self.process_command_during_search(command),
            PromptType::Save => self.process_command_during_save(command),
            PromptType::None => self.process_command_no_prompt(command),
        }
    }

    fn process_command_no_prompt(&mut self, command: Command) {
        if matches!(command, System(Quit)) {
            self.handle_quit_command();
            return;
        }
        self.reset_quit_times();
        match command {
            System(Quit | Resize(_) | Dismiss) => {}
            System(Search) => self.set_prompt(PromptType::Search),
            System(Save) => self.handle_save_command(),
            Edit(edit_command) => self.view.handle_edit_command(edit_command),
            Move(move_command) => self.view.handle_move_command(move_command),
        }
    }

    fn handle_resize_command(&mut self, size: Size) {
        self.terminal_size = size;
        self.view.resize(Size {
            height: size.height.saturating_sub(2),
            width: size.width,
        });
        let bar_size = Size {
            height: 1,
            width: size.width,
        };
        self.message_bar.resize(bar_size);
        self.status_bar.resize(bar_size);
        self.command_bar.resize(bar_size);
    }

    fn handle_quit_command(&mut self) {
        if !self.view.get_status().is_modified || self.quit_times + 1 == QUIT_TIMES {
            self.should_quit = true;
        } else if self.view.get_status().is_modified {
            self.update_message(format!(
                "Warning! File has unsaved changes. Press Ctrl-Q {} more times to quit",
                QUIT_TIMES - self.quit_times - 1
            ));
            self.quit_times += 1;
        }
    }

    fn handle_save_command(&mut self) {
        if self.view.is_file_loaded() {
            self.save(None);
        } else {
            self.set_prompt(PromptType::Save);
        }
    }

    fn process_command_during_save(&mut self, command: Command) {
        match command {
            System(Quit | Resize(_) | Search | Save) | Move(_) => {} // can not be handled during save
            System(Dismiss) => {
                self.set_prompt(PromptType::None);
                self.update_message("Save aborted".to_string());
            }
            Edit(InsertNewline) => {
                let file_name = self.command_bar.value();
                self.save(Some(&file_name));
                self.set_prompt(PromptType::None);
            }
            Edit(edit_command) => self.command_bar.handle_edit_command(edit_command),
        }
    }

    fn save(&mut self, file_name: Option<&str>) {
        let result = if let Some(name) = file_name {
            self.view.save_as(name)
        } else {
            self.view.save()
        };
        if result.is_ok() {
            self.update_message("File saved successfully".to_string());
        } else {
            self.update_message("Error saving file".to_string());
        }
    }

    fn process_command_during_search(&mut self, command: Command) {
        match command {
            System(System::Dismiss) | Edit(Edit::InsertNewline) => {
                self.set_prompt(PromptType::None);
                self.view.dismiss_search();
            }
            Edit(edit_command) => {
                self.command_bar.handle_edit_command(edit_command);
                let query = self.command_bar.value();
                self.view.search(&query);
            }
            Move(Move::Right | Move::Down) => self.view.search_next(),
            Move(Move::Up | Move::Left) => self.view.search_prev(),
            System(System::Quit | Resize(_) | System::Search | System::Save) | Move(_) => {} // can not be handled during search
        }
    }

    fn update_message(&mut self, message: String) {
        self.message_bar.update_message(message);
    }

    fn in_prompt(&self) -> bool {
        !self.prompt_type.is_none()
    }

    fn set_prompt(&mut self, prompt_type: PromptType) {
        match prompt_type {
            PromptType::None => self.message_bar.set_needs_redraw(true),
            PromptType::Save => self.command_bar.set_prompt("Save as: "),
            PromptType::Search => {
                self.view.enter_search();
                self.command_bar
                    .set_prompt("Search: (Esc to cancel, Arrows to navigate): ");
            }
        }
        self.command_bar.clean_value();
        self.prompt_type = prompt_type;
    }

    fn reset_quit_times(&mut self) {
        if self.quit_times > 0 {
            self.quit_times = 0;
            self.update_message("".to_string());
        }
    }

    pub fn run(&mut self) {
        loop {
            self.refresh_screen();
            if self.should_quit {
                break;
            }

            match read() {
                Ok(event) => self.evaluate_event(event),
                Err(err) => {
                    panic!("could not read event {err:?}");
                }
            }
            let status = self.view.get_status();
            self.status_bar.update_status(status);
        }
    }

    fn evaluate_event(&mut self, event: crossterm::event::Event) {
        let should_process = match &event {
            Event::Key(KeyEvent { kind, .. }) => kind == &KeyEventKind::Press,
            Event::Resize(_, _) => true,
            _ => false,
        };

        if should_process {
            if let Ok(command) = Command::try_from(event) {
                self.process_command(command);
            }
        } else {
            panic!("Event not supported");
        }
    }
}
