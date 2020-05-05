//!
#![warn(missing_debug_implementations, rust_2018_idioms, missing_docs)]

pub struct StrSplit<'a> {
    remainder: &'a str,
    delimiter: &'a str,
}

impl StrSplit {
    pub fn new(heystack: &str, delimiter: &str) -> Self {
        Self {
            remainder: heystack,
            delimiter: delimiter,
        }
    }
}

impl Iterator for StrSplit {
    type Item = &str;

    fn next(&mut self) -> Option<Self::Item> {

    }

}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let heystack = "a b c d e";
        let letters = StrSplit::new(heystack, " ");
        assert_eq!(letters, vec!["a", "b", "c", "d", "e"].into_iter());
    }
}

