use crate::{Correctness, Guess, Guesser};

pub struct Naive;

impl Naive {
    pub fn new() -> Self {
        Naive
    }
}

impl Guesser for Naive {
    fn guess(&mut self, history: &[Guess]) -> String {
        todo!();
    }
}
