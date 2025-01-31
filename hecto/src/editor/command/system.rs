use crossterm::event::{
    KeyCode::{self, Char}, 
    KeyEvent, 
    KeyModifiers,
};

use crate::editor::size::Size;

#[derive(Debug, Clone, Copy)]
pub enum System {
    Save,
    Resize(Size),
    Quit,
    Search,
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
                Char('f') => Ok(Self::Search),
                _ => Err(format!("Key Code not supported: {code:?}")),
            }
        } else if modifiers == KeyModifiers::NONE && matches!(code, KeyCode::Esc) {
            Ok(Self::Dismiss)
        } else {
            Err("Key Modifiers not supported".to_string())
        }
    }
}