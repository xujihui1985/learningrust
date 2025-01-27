use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};

use crate::editor::terminal::Size;

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    PageUp,
    PageDown,
    Home,
    End,
    Up,
    Left,
    Right,
    Down,
}

#[derive(Debug, Clone, Copy)]
pub enum EditorCommand {
    Move(Direction),
    Resize(Size),
    Quit,
    Insert(char),
    Backspace,
    Delete,
    Enter,
    Save,
}

impl TryFrom<Event> for EditorCommand {
    type Error = String;

    fn try_from(event: Event) -> Result<Self, Self::Error> {
        match event {
            Event::Key(KeyEvent{
                code,modifiers,..
            }) => match (code, modifiers) {
                (KeyCode::Char('q'), KeyModifiers::CONTROL) => Ok(Self::Quit),
                (KeyCode::Char(character), KeyModifiers::NONE | KeyModifiers::SHIFT) => Ok(Self::Insert(character)),
                (KeyCode::Char('s'), KeyModifiers::CONTROL) => Ok(Self::Save),
                (KeyCode::Up, _) => Ok(Self::Move(Direction::Up)),
                (KeyCode::Down, _) => Ok(Self::Move(Direction::Down)),
                (KeyCode::Left, _) => Ok(Self::Move(Direction::Left)),
                (KeyCode::Right, _) => Ok(Self::Move(Direction::Right)),
                (KeyCode::PageDown, _) => Ok(Self::Move(Direction::PageDown)),
                (KeyCode::PageUp, _) => Ok(Self::Move(Direction::PageUp)),
                (KeyCode::Home, _) => Ok(Self::Move(Direction::Home)),
                (KeyCode::End, _) => Ok(Self::Move(Direction::End)),
                (KeyCode::Backspace, _) => Ok(Self::Backspace),
                (KeyCode::Delete, _) => Ok(Self::Delete),
                (KeyCode::Enter, _) => Ok(Self::Enter),
                (KeyCode::Tab, _) => Ok(Self::Insert('\t')),
                _ => Err(format!("Key Code not supported: {code:?}")),
            }
            Event::Resize(width,height ) => {
                let height = height as usize;
                let width = width as usize;
                Ok(Self::Resize(Size { height, width }))
            }
            _ =>   Err("Event not supported".to_string())
        }
    }
}