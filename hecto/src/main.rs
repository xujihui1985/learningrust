#![warn(clippy::all, clippy::pedantic)]

mod editor;
mod prelude;

fn main() {
    let mut e = editor::Editor::new().unwrap();
    e.run();
}
