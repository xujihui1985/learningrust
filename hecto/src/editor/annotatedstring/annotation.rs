use super::AnnotationType;

#[derive(Debug, Clone, Copy)]
pub struct Annotation {
    pub annotation_type: AnnotationType,
    pub start_byte_idx: usize,
    pub end_byte_idx: usize,
}