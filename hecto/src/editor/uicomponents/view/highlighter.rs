use std::collections::HashMap;

use super::location::Location;
use crate::{editor::{line::Line, Annotation, AnnotationType}, prelude::*};

#[derive(Default)]
pub struct Highlighter<'a> {
    matched_word: Option<&'a str>,
    selected_match: Option<Location>,
    highlights: HashMap<LineIdx, Vec<Annotation>>,
}

impl<'a> Highlighter<'a> {
    pub fn new(matched_word: Option<&'a str>, selected_match: Option<Location>) -> Self {
        Self {
            matched_word,
            selected_match,
            highlights: HashMap::new(),
        }
    }

    pub fn get_annotations(&self, idx: LineIdx) -> Option<&Vec<Annotation>> {
        self.highlights.get(&idx)
    }

    fn highlight_digits(line: &Line, result: &mut Vec<Annotation>) {
        line.chars().enumerate().for_each(|(idx, ch)| {
            if ch.is_ascii_digit() {
                result.push(Annotation{
                    start_byte_idx: idx,
                    end_byte_idx: idx.saturating_add(1),
                    annotation_type: AnnotationType::Digit,
                });
            }
        });
    }

    fn highlight_matched_words(&self, line: &Line, result: &mut Vec<Annotation>) {
        if let Some(matched_word) = self.matched_word {
            if matched_word.is_empty() {
                return;
            }
            line.find_all(matched_word, 0..line.len())
                .iter()
                .for_each(|(start, _)| {
                    result.push(Annotation {
                        annotation_type: AnnotationType::Match,
                        start_byte_idx: *start,
                        end_byte_idx: start.saturating_add(matched_word.len()),
                    });
                });
        }
    }

    fn highlight_selected_match(&self, result: &mut Vec<Annotation>) {
        if let Some(selected_match) = self.selected_match {
            if let Some(matched_word) = self.matched_word {
                if matched_word.is_empty() {
                    return;
                }
                let start = selected_match.grapheme_index;
                result.push(Annotation {
                    annotation_type: AnnotationType::SelectedMatch,
                    start_byte_idx: start,
                    end_byte_idx: start.saturating_add(matched_word.len()),
                });
            }
        }
    }

    pub fn highlight(&mut self, idx: LineIdx, line: &Line) {
        let mut result = Vec::new();
        Self::highlight_digits(line, &mut result);
        self.highlight_matched_words(line, &mut result);
        if let Some(seleted_match) = self.selected_match {
            if seleted_match.line_index == idx {
                self.highlight_selected_match(&mut result);
            }
        }
        self.highlights.insert(idx, result);
    }
}
