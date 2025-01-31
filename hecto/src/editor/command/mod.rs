
mod movecommand;
mod system;
mod edit;

use crossterm::event::Event;
pub use edit::Edit;
pub use movecommand::Move;
pub use system::System;

use crate::editor::size::Size;

#[derive(Debug, Clone, Copy)]
pub enum Command {
    Move(Move),
    Edit(Edit),
    System(System),
}

impl TryFrom<Event> for Command {
    type Error = String;

    fn try_from(value: Event) -> Result<Self, Self::Error> {
        match value {
            Event::Key(key_event) => Edit::try_from(key_event)
                .map(|op| Command::Edit(op))
                .or_else(|_| Move::try_from(key_event).map(Command::Move))
                .or_else(|_| System::try_from(key_event).map(Command::System))
                .map_err(|err| format!("Error: {err}")),
            Event::Resize(width_u16, height_u16) => Ok(Self::System(System::Resize(Size {
                width: width_u16 as usize,
                height: height_u16 as usize,
            }))),
            _ => Err("Event not supported".to_string()),
        }
    }
}
