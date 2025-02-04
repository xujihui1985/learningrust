use std::collections::HashMap;

use syntaxhighlighter::SyntaxHighlighter;

use super::location::Location;
use crate::{
    editor::{annotation, filetype::FileType, line::Line, Annotation, AnnotationType},
    prelude::*,
};

mod rustsyntaxhighlighter;
mod searchresulthighlighter;
mod syntaxhighlighter;

use rustsyntaxhighlighter::RustSyntaxHighlighter;
use searchresulthighlighter::SearchResultHighlighter;

fn create_syntax_highlighter(file_type: FileType) -> Option<Box<dyn SyntaxHighlighter>> {
    match file_type {
        FileType::Rust => Some(Box::new(RustSyntaxHighlighter::default())),
        FileType::Text => None,
    }
}

#[derive(Default)]
pub struct Highlighter<'a> {
    syntax_highlighter: Option<Box<dyn SyntaxHighlighter>>,
    search_result_highlighter: Option<SearchResultHighlighter<'a>>,
}

impl<'a> Highlighter<'a> {
    pub fn new(
        matched_word: Option<&'a str>, 
        selected_match: Option<Location>,
        file_type: FileType,
    ) -> Self {
        let search_result_hl =
            matched_word.map(|mw| SearchResultHighlighter::new(mw, selected_match));
        Self {
            search_result_highlighter: search_result_hl,
            syntax_highlighter: create_syntax_highlighter(file_type)
        }
    }

    pub fn get_annotations(&self, idx: LineIdx) -> Vec<Annotation> {
        let mut result = Vec::new();
        if let Some(syntax_highlighter) = &self.syntax_highlighter {
            if let Some(annotations) = syntax_highlighter.get_annotations(idx) {
                result.extend_from_slice(annotations);
            }
        }
        if let Some(search_result_hl) = &self.search_result_highlighter {
            if let Some(annotations) = search_result_hl.get_annotations(idx) {
                result.extend_from_slice(annotations);
            }
        }
        result
    }

    // fn highlight_digits(line: &Line, result: &mut Vec<Annotation>) {
    //     line.chars().enumerate().for_each(|(idx, ch)| {
    //         if ch.is_ascii_digit() {
    //             result.push(Annotation {
    //                 start_byte_idx: idx,
    //                 end_byte_idx: idx.saturating_add(1),
    //                 annotation_type: AnnotationType::Digit,
    //             });
    //         }
    //     });
    // }

    // fn highlight_matched_words(&self, line: &Line, result: &mut Vec<Annotation>) {
    //     if let Some(matched_word) = self.matched_word {
    //         if matched_word.is_empty() {
    //             return;
    //         }
    //         line.find_all(matched_word, 0..line.len())
    //             .iter()
    //             .for_each(|(start, _)| {
    //                 result.push(Annotation {
    //                     annotation_type: AnnotationType::Match,
    //                     start_byte_idx: *start,
    //                     end_byte_idx: start.saturating_add(matched_word.len()),
    //                 });
    //             });
    //     }
    // }

    // fn highlight_selected_match(&self, result: &mut Vec<Annotation>) {
    //     if let Some(selected_match) = self.selected_match {
    //         if let Some(matched_word) = self.matched_word {
    //             if matched_word.is_empty() {
    //                 return;
    //             }
    //             let start = selected_match.grapheme_index;
    //             result.push(Annotation {
    //                 annotation_type: AnnotationType::SelectedMatch,
    //                 start_byte_idx: start,
    //                 end_byte_idx: start.saturating_add(matched_word.len()),
    //             });
    //         }
    //     }
    // }

    pub fn highlight(&mut self, idx: LineIdx, line: &Line) {
        if let Some(syntax_highlighter) = &mut self.syntax_highlighter {
            syntax_highlighter.highlight(idx, line);
        }
        if let Some(search_result_hl) = &mut self.search_result_highlighter {
            search_result_hl.highlight(idx, line);
        }
        // let mut result = Vec::new();
        // Self::highlight_digits(line, &mut result);
        // self.highlight_matched_words(line, &mut result);
        // if let Some(seleted_match) = self.selected_match {
        //     if seleted_match.line_index == idx {
        //         self.highlight_selected_match(&mut result);
        //     }
        // }
        // self.highlights.insert(idx, result);
    }
}
