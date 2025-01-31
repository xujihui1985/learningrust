use crossterm::event::{
    KeyCode::{Backspace, Char, Delete, Enter, Tab},
    KeyEvent, KeyModifiers,
};

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
