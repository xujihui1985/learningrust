use crossterm::style::Color;

use crate::editor::AnnotationType;

pub struct Attribute {
    pub foreground: Option<Color>,
    pub background: Option<Color>,
}

impl From<AnnotationType> for Attribute {
    fn from(value: AnnotationType) -> Self {
        match value {
            AnnotationType::Match => Self {
                foreground: Some(Color::Rgb {
                    r: 255,
                    g: 255,
                    b: 255,
                }),
                background: Some(Color::Rgb {
                    r: 100,
                    g: 100,
                    b: 100,
                }),
            },
            AnnotationType::SelectedMatch => Self {
                foreground: Some(Color::Rgb {
                    r: 255,
                    g: 255,
                    b: 255,
                }),
                background: Some(Color::Rgb {
                    r: 255,
                    g: 251,
                    b: 0,
                }),
            },
            AnnotationType::Number => Self {
                foreground: Some(Color::Rgb {
                    r: 255,
                    g: 99,
                    b: 71,
                }),
                background: None,
            },
            AnnotationType::Keyword => Self {
                foreground: Some(Color::Rgb {
                    r: 100,
                    g: 149,
                    b: 237,
                }),
                background: None,
            },
            AnnotationType::Type => Self {
                foreground: Some(Color::Rgb {
                    r: 175,
                    g: 225,
                    b: 175,
                }),
                background: None,
            },
            AnnotationType::KnownValue => Self {
                foreground: Some(Color::Rgb {
                    r: 195,
                    g: 177,
                    b: 225,
                }),
                background: None,
            },
            AnnotationType::Char => Self {
                foreground: Some(Color::Rgb {
                    r: 255,
                    g: 191,
                    b: 0,
                }),
                background: None,
            },
            AnnotationType::LifetimeSpecifier => Self {
                foreground: Some(Color::Rgb {
                    r: 102,
                    g: 205,
                    b: 170,
                }),
                background: None,
            },
            AnnotationType::Comment => Self {
                foreground: Some(Color::Rgb {
                    r: 34,
                    g: 139,
                    b: 34,
                }),
                background: None,
            },
            AnnotationType::String => Self {
                foreground: Some(Color::Rgb {
                    r: 255,
                    g: 179,
                    b: 102,
                }),
                background: None,
            },
        }
    }
}
