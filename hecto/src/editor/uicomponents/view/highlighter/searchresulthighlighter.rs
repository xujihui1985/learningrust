use std::collections::HashMap;

use crate::editor::{
    annotation::Annotation, line::Line, uicomponents::view::location::Location, AnnotationType,
};

use super::{syntaxhighlighter::SyntaxHighlighter, LineIdx};

#[derive(Default)]
pub struct SearchResultHighlighter<'a> {
    matched_word: &'a str,
    selected_match: Option<Location>,
    highlights: HashMap<LineIdx, Vec<Annotation>>,
}

impl<'a> SearchResultHighlighter<'a> {
    pub fn new(matched_word: &'a str, selected_match: Option<Location>) -> Self {
        Self {
            matched_word,
            selected_match,
            highlights: HashMap::new(),
        }
    }

    fn highlight_matched_words(&self, line: &Line, result: &mut Vec<Annotation>) {
        if self.matched_word.is_empty() {
            return;
        }

        line.find_all(self.matched_word, 0..line.len())
            .iter()
            .for_each(|(start, _)| {
                result.push(Annotation {
                    annotation_type: AnnotationType::Match,
                    start_byte_idx: *start,
                    end_byte_idx: start.saturating_add(self.matched_word.len()),
                });
            });
    }

    fn highlight_selected_match(&self, result: &mut Vec<Annotation>) {
        if let Some(selected_match) = self.selected_match {
            if self.matched_word.is_empty() {
                return;
            }
            let start = selected_match.grapheme_index;
            result.push(Annotation {
                annotation_type: AnnotationType::SelectedMatch,
                start_byte_idx: start,
                end_byte_idx: start.saturating_add(self.matched_word.len()),
            });
        }
    }
}

impl<'a> SyntaxHighlighter for SearchResultHighlighter<'a> {
    fn highlight(&mut self, idx: LineIdx, line: &Line) {
        let mut result = Vec::new();
        self.highlight_matched_words(line, &mut result);
        if let Some(selected_match) = self.selected_match {
            if selected_match.line_index == idx {
                self.highlight_selected_match(&mut result);
            }
        }
    }

    fn get_annotations(&self, idx: LineIdx) -> Option<&Vec<Annotation>> {
        self.highlights.get(&idx)
    }
}
