use crossterm::event::{
    Event,
    KeyCode::{self, *},
};
use crossterm::event::{KeyEvent, KeyModifiers};

use super::Size;

#[derive(Debug, Clone, Copy)]
pub enum Move {
    PageUp,
    PageDown,
    StartOfLine,
    EndOfLine,
    Up,
    Left,
    Right,
    Down,
}

impl TryFrom<KeyEvent> for Move {
    type Error = String;

    fn try_from(value: KeyEvent) -> Result<Self, Self::Error> {
        let KeyEvent {
            code, modifiers, ..
        } = value;

        if modifiers == KeyModifiers::NONE {
            match code {
                Up => Ok(Self::Up),
                Down => Ok(Self::Down),
                Left => Ok(Self::Left),
                Right => Ok(Self::Right),
                PageDown => Ok(Self::PageDown),
                PageUp => Ok(Self::PageUp),
                Home => Ok(Self::StartOfLine),
                End => Ok(Self::EndOfLine),
                _ => Err(format!("Key Code not supported: {code:?}")),
            }
        } else {
            Err("Key Modifiers not supported".to_string())
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Edit {
    Insert(char),
    InsertNewline,
    Delete,
    DeleteBackward,
}

impl TryFrom<KeyEvent> for Edit {
    type Error = String;

    fn try_from(value: KeyEvent) -> Result<Self, Self::Error> {
        match (value.code, value.modifiers) {
            (Char(character), KeyModifiers::NONE | KeyModifiers::SHIFT) => {
                Ok(Self::Insert(character))
            }
            (Tab, KeyModifiers::NONE) => Ok(Self::Insert('\t')),
            (Enter, KeyModifiers::NONE) => Ok(Self::InsertNewline),
            (Backspace, KeyModifiers::NONE) => Ok(Self::DeleteBackward),
            (Delete, KeyModifiers::NONE) => Ok(Self::Delete),
            _ => Err("Key Code not supported".to_string()),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum System {
    Save,
    Resize(Size),
    Quit,
    Dismiss,
}

impl TryFrom<KeyEvent> for System {
    type Error = String;

    fn try_from(value: KeyEvent) -> Result<Self, Self::Error> {
        let KeyEvent {
            code, modifiers, ..
        } = value;

        if modifiers == KeyModifiers::CONTROL {
            match code {
                Char('s') => Ok(Self::Save),
                Char('q') => Ok(Self::Quit),
                _ => Err(format!("Key Code not supported: {code:?}")),
            }
        } else if modifiers == KeyModifiers::NONE && matches!(code, KeyCode::Esc) {
            Ok(Self::Dismiss)
        } else {
            Err("Key Modifiers not supported".to_string())
        }
    }
}

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
