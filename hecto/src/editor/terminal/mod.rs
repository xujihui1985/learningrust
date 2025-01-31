use super::annotatedstring::AnnotatedString;
use super::{Position, Size};
use crossterm::style::Attribute;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, size, Clear, ClearType, EnterAlternateScreen,
    LeaveAlternateScreen,
};
use crossterm::{queue, Command};
use std::io::{stdout, Error as IoError, Write};

mod attribute;

type Result<T> = std::result::Result<T, IoError>;

pub struct Terminal {}

impl Terminal {
    pub fn terminate() -> Result<()> {
        Self::leave_alternate_screen()?;
        Self::show_caret()?;
        Self::execute()?;
        disable_raw_mode()
    }

    pub fn initialize() -> Result<()> {
        enable_raw_mode()?;
        Self::enter_alternate_screen()?;
        Self::clear_screen()?;
        Self::move_caret_to(Position::default())?;
        Self::execute()?;
        Ok(())
    }

    pub fn disable_line_wrap() -> Result<()> {
        Self::queue_command(crossterm::terminal::DisableLineWrap)
    }

    pub fn enable_line_wrap() -> Result<()> {
        Self::queue_command(crossterm::terminal::EnableLineWrap)
    }

    pub fn set_title(title: &str) -> Result<()> {
        Self::queue_command(crossterm::terminal::SetTitle(title))
    }

    pub fn clear_screen() -> Result<()> {
        Self::queue_command(Clear(ClearType::All))
    }

    pub fn clear_line() -> Result<()> {
        Self::queue_command(Clear(ClearType::CurrentLine))
    }

    pub fn move_caret_to(position: Position) -> Result<()> {
        Self::queue_command(crossterm::cursor::MoveTo(
            position.col as u16,
            position.row as u16,
        ))
    }

    pub fn hide_caret() -> Result<()> {
        Self::queue_command(crossterm::cursor::Hide)
    }

    pub fn show_caret() -> Result<()> {
        Self::queue_command(crossterm::cursor::Show)
    }

    pub fn print(string: &str) -> Result<()> {
        Self::queue_command(crossterm::style::Print(string))
    }

    pub fn print_annotated_row(row: usize, annotated_string: &AnnotatedString) -> Result<()> {
        Self::move_caret_to(Position { row, col: 0 })?;
        Self::clear_line()?;
        annotated_string.into_iter().try_for_each(|part| -> Result<()> {
            if let Some(anno_type) = part.annotation_type {
                let attr = attribute::Attribute::from(anno_type);
                Self::set_attribute(attr)?;
            }
            Self::print(part.string)?;
            Self::reset_color()?;
            Ok(())
        })?;
        Ok(())
    }

    fn set_attribute(attribute: attribute::Attribute) -> Result<()> {
        if let Some(forg_color) = attribute.foreground {
            Self::queue_command(crossterm::style::SetForegroundColor(forg_color))?;
        }
        if let Some(bg_color) = attribute.background {
            Self::queue_command(crossterm::style::SetBackgroundColor(bg_color))?;
        }
        Ok(())
    }

    fn reset_color() -> Result<()> {
        Self::queue_command(crossterm::style::ResetColor)
    }

    pub fn print_inverted_row(row: usize, line_text: &str) -> Result<()> {
        let width = Self::size()?.width;
        Self::print_row(
            row,
            &format!(
                "{}{:width$.width$}{}",
                Attribute::Reverse,
                line_text,
                Attribute::Reset
            ),
        )
    }

    pub fn size() -> Result<Size> {
        let (width, height) = size()?;
        #[allow(clippy::as_conversions)]
        Ok(Size {
            width: width as usize,
            height: height as usize,
        })
    }

    pub fn execute() -> Result<()> {
        stdout().flush()
    }

    fn queue_command<T: Command>(command: T) -> Result<()> {
        queue!(stdout(), command)
    }

    pub fn enter_alternate_screen() -> Result<()> {
        Self::queue_command(EnterAlternateScreen)
    }

    pub fn leave_alternate_screen() -> Result<()> {
        Self::queue_command(LeaveAlternateScreen)
    }

    pub fn print_row(row: usize, line_text: &str) -> Result<()> {
        Self::move_caret_to(Position { row, col: 0 })?;
        Self::clear_line()?;
        Self::print(line_text)?;
        Ok(())
    }
}
