#![warn(clippy::all, clippy::pedantic)]

mod editor;

fn main() {
    let mut e = editor::Editor::new().unwrap();
    e.run();
}
