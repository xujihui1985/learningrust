//!
#![warn(missing_debug_implementations, rust_2018_idioms, missing_docs)]

#[derive(Debug)]
pub struct StrSplit<'a> {
    remainder: &'a str,
    delimiter: &'a str,
}

impl<'a> StrSplit<'a> {
    // aaaa
    pub fn new(heystack: &'a str, delimiter: &'a str) -> Self {
        Self {
            remainder: heystack,
            delimiter,
        }
    }
}

impl<'a> Iterator for StrSplit<'a> {
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
