use command::{Command, System};
use commandbar::CommandBar;
use crossterm::event::{read, Event, KeyEvent, KeyEventKind};
use messagebar::MessageBar;
use statusbar::StatusBar;
use std::env;
use std::io::Error as IoError;
use std::panic::{set_hook, take_hook};
pub(crate) mod command;
pub(crate) mod commandbar;
pub(crate) mod documentstatus;
pub(crate) mod line;
pub(crate) mod messagebar;
pub(crate) mod position;
pub(crate) mod size;
pub(crate) mod statusbar;
pub(crate) mod terminal;
pub(crate) mod uicomponent;
mod view;
use crate::editor::Command::*;
use crate::editor::System::*;
use line::Line;
use position::Position;
use size::Size;
use terminal::Terminal;
use uicomponent::UIComponent;
use view::View;
// use command::{System, Command};

pub const NAME: &str = env!("CARGO_PKG_NAME");
const HELP_MESSAGE: &str = "Help: Ctrl-S = save | Ctrl-Q = quit";
const QUIT_TIMES: u8 = 3;

type Result<T> = std::result::Result<T, IoError>;

#[derive(Default)]
pub struct Editor {
    should_quit: bool,
    view: View,
    status_bar: StatusBar,
    message_bar: MessageBar,
    command_bar: Option<CommandBar>,
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
        editor.resize(size);
        editor.message_bar.update_message(HELP_MESSAGE.to_string());

        let args = env::args().collect::<Vec<_>>();
        if let Some(file_name) = args.get(1) {
            if editor.view.load(file_name).is_err() {
                editor
                    .message_bar
                    .update_message(format!("Could not open file: {}", file_name));
            }
        }
        editor.refresh_screen();
        Ok(editor)
    }

    fn process_command(&mut self, command: Command) {
        match command {
            System(Quit) => {
                if self.command_bar.is_none() {
                    self.handle_quit();
                }
            }
            System(Resize(size)) => self.resize(size),
            _ => self.reset_quit_times(), // Reset quit times for all other commands
        }
        match command {
            System(Quit | Resize(_)) => {} // already handled above
            System(Save) => {
                if self.command_bar.is_none() {
                    self.handle_save();
                }
            }
            System(Dismiss) => {
                if self.command_bar.is_some() {
                    self.dismiss_prompt();
                    self.message_bar.update_message("Save aborted".to_string());
                }
            }
            Edit(edit_command) => {
                if let Some(command_bar) = &mut self.command_bar {
                    if matches!(edit_command, command::Edit::InsertNewline) {
                        let file_name = command_bar.value();
                        self.dismiss_prompt();
                        self.save(Some(&file_name));
                    } else {
                        command_bar.handle_edit_command(edit_command);
                    }
                } else {
                    self.view.handle_edit_command(edit_command);
                }
            }
            Move(move_command) => {
                if self.command_bar.is_none() {
                    self.view.handle_move_command(move_command);
                }
            }
            _ => todo!(),
        }
    }

    fn dismiss_prompt(&mut self) {
        self.command_bar = None;
        self.message_bar.set_needs_redraw(true);
    }

    fn show_prompt(&mut self) {
        let mut command_bar = CommandBar::default();
        command_bar.set_prompt("Save as: ");
        command_bar.resize(Size{
            height: 1,
            width: self.terminal_size.width,
        });
        command_bar.set_needs_redraw(true);
        self.command_bar = Some(command_bar);
    }

    fn handle_save(&mut self) {
        if self.view.is_file_loaded() {
            self.save(None);
        } else {
            self.show_prompt();
        }
    }

    fn save(&mut self, file_name: Option<&str>) {
        let result = if let Some(name) = file_name {
            self.view.save_as(name)
        } else {
            self.view.save()
        };
        if result.is_ok() {
            self.message_bar.update_message("File saved successfully".to_string());
        } else {
            self.message_bar.update_message("Error saving file".to_string());
        }
    }

    fn handle_quit(&mut self) {
        if !self.view.get_status().is_modified || self.quit_times + 1 == QUIT_TIMES {
            self.should_quit = true;
        } else if self.view.get_status().is_modified {
            self.message_bar.update_message(format!(
                "Warning! File has unsaved changes. Press Ctrl-Q {} more times to quit",
                QUIT_TIMES - self.quit_times - 1
            ));
            self.quit_times += 1;
        }
    }

    fn reset_quit_times(&mut self) {
        if self.quit_times > 0 {
            self.quit_times = 0;
            self.message_bar.update_message("".to_string());
        }
    }

    pub fn resize(&mut self, size: Size) {
        self.terminal_size = size;
        self.view.resize(Size {
            height: size.height.saturating_sub(2),
            width: size.width,
        });
        self.message_bar.resize(Size {
            height: 1,
            width: size.width,
        });
        self.status_bar.resize(Size {
            height: 1,
            width: size.width,
        });
        if let Some(command_bar) = &mut self.command_bar {
            command_bar.resize(Size {
                height: 1,
                width: size.width,
            });
        }
    }

    pub fn refresh_status(&mut self) {
        let status = self.view.get_status();
        let title = format!("{} - {NAME}", status.file_name,);
        self.status_bar.update_status(status);
        if title != self.title && matches!(Terminal::set_title(&title), Ok(())) {
            self.title = title;
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

    fn refresh_screen(&mut self) {
        if self.terminal_size.height == 0 || self.terminal_size.width == 0 {
            return;
        }

        let bottom_bar_row = self.terminal_size.height.saturating_sub(1);
        let _ = Terminal::hide_caret();
        if let Some(command_bar) = &mut self.command_bar {
            command_bar.render(bottom_bar_row);
        } else {
            self.message_bar.render(bottom_bar_row);
        }

        self.message_bar
            .render(self.terminal_size.height.saturating_sub(1));
        if self.terminal_size.height > 1 {
            self.status_bar
                .render(self.terminal_size.height.saturating_sub(2));
        }
        if self.terminal_size.height > 2 {
            self.view.render(0);
        }
        let new_caret_pos = if let Some(command_bar) = &self.command_bar {
            Position {
                row: bottom_bar_row,
                col: command_bar.caret_position_col(),
            }
        } else {
            self.view.caret_position()
        };
        let _ = Terminal::move_caret_to(new_caret_pos);
        let _ = Terminal::show_caret();
        let _ = Terminal::execute();
    }
}
