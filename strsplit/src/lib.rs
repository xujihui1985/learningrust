//!
#![warn(missing_debug_implementations, rust_2018_idioms, missing_docs)]

use std::str::pattern::Pattern;

#[derive(Debug)]
pub struct StrSplit<'a, 'b> {
    remainder: &'a str,
    delimiter: &'b str,
}

impl<'a, 'b> StrSplit<'a, 'b> {
    // aaaa
    pub fn new(heystack: &'a str, delimiter: &'b str) -> Self {
        Self {
            remainder: heystack,
            delimiter,
        }
    }
}

impl<'a,'b> Iterator for StrSplit<'a,'b> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(next_delim) = self.remainder.find(self.delimiter) {
            let until_delimiter = &self.remainder[..next_delim];
            self.remainder = &self.remainder[(next_delim + self.delimiter.len())..];
            Some(until_delimiter)
        } else if self.remainder.is_empty() {
            None
        } else {
            let rest = self.remainder;
            self.remainder = "";
            Some(rest)
        }
    }

}

fn until_char(s: &str, c: char) -> Option<&str> {
    let delimter = format!("{}", c);
    StrSplit::new(s, &delimter).next()
}
fn run_case(content: &String, query: &String) {
    // query.is_contained_in("xx");
    "xxx".is_contained_in("xx");
    for line in content.lines() {
        if line.contains(query) {
            println!("{}", line);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::StrSplit;

    #[test]
    fn it_works() {
        let heystack = "a b c d e";
        let letters = StrSplit::new(heystack, " ");
        assert!(letters.eq(vec!["a", "b", "c", "d", "e"].into_iter()));

    }
}
