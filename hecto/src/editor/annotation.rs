use super::AnnotationType;

#[derive(Debug, Clone, Copy)]
pub struct Annotation {
    pub annotation_type: AnnotationType,
    pub start_byte_idx: usize,
    pub end_byte_idx: usize,
}

impl Annotation {
    pub fn shift(&mut self, offset: usize) {
        self.start_byte_idx = self.start_byte_idx.saturating_add(offset);
        self.end_byte_idx = self.end_byte_idx.saturating_add(offset);
    }
}