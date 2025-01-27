use super::Size;

pub trait UIComponent {

    fn set_needs_redraw(&mut self, value: bool);
    fn needs_redraw(&self) -> bool;
    fn set_size(&mut self, size: Size);
    fn draw(&mut self, origin_row: usize) -> Result<(), std::io::Error>;


    fn resize(&mut self, size: Size) {
        self.set_size(size);
        self.set_needs_redraw(true);
    }

    fn render(&mut self, origin_row: usize) {
        if self.needs_redraw() {
            match self.draw(origin_row) {
                Ok(_) => self.set_needs_redraw(false),
                Err(e) => {
                    panic!("Failed to draw UI component: {e:?}");
                }
            }
        }
    }
}