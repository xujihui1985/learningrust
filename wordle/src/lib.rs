use std::collections::HashSet;

pub mod myiter;
pub mod algorithms;

const DICTIONARY: &str = include_str!("../dictionary.txt");

pub struct Wordle {
    dictionary: HashSet<&'static str>,
}

impl Wordle {
    pub fn new() -> Self {
        Self {
            dictionary: HashSet::from_iter(DICTIONARY.lines().map(|line| {
                line.split_once(' ').expect("aaaaa").1
            }))
        }
    }

    pub fn play<G: Guesser>(&self, answer: &'static str, mut guesser: G) -> Option<usize> {
        let mut history = Vec::new();
        for i in 1..=32 {
            let guess = guesser.guess(&history[..]);
            if guess == answer { 
                return Some(i);
            }
            assert!(self.dictionary.contains(&*guess));
            let correctness = Correctness::compute(answer, &guess);
            history.push(Guess {
                word: guess, 
                mask: correctness,
            });
        }
        None
    }
}

impl Correctness {
    fn compute(answer: &str, guess: &str) -> [Self; 5] {
        assert_eq!(answer.len(), 5);
        assert_eq!(guess.len(), 5);

        // initially all correctness are wrong
        let mut c = [Correctness::Wrong; 5];
        for (i, (a, g)) in answer.chars().zip(guess.chars()).enumerate() {
            if a == g {
                c[i] = Correctness::Correct;
            }
        }
        let mut used = [false; 5];
        for (i, &c) in c.iter().enumerate() {
            if c == Correctness::Correct {
                used[i] = true;
            }
        }

        for (i, g) in guess.chars().enumerate() {
            if c[i] == Correctness::Correct {
                continue;
            }
            if answer.chars().enumerate().any(|(i, a)| {
                if a == g && !used[i] {
                    used[i] = true;
                    return true;
                }
                false
            }) {
                c[i] = Correctness::Misplaced;
            }
        }
        c
    }
}

fn check(answer: &str, guess: &str) -> [Correctness; 5] {
    todo!()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Correctness {
    Correct,
    Misplaced,
    Wrong,
}

pub struct Guess{
    pub word: String, 
    pub mask: [Correctness; 5],
}

pub trait Guesser {
    fn guess(&mut self, history: &[Guess]) -> String;
}

#[cfg(test)]
macro_rules! guesser {
    (|$history:ident| $impl:block) => {{
        struct G;
        impl $crate::Guesser for G {
            fn guess(&mut self, $history: &[Guess]) -> String {
                $impl
            }
        }
        G
    }};
}


#[cfg(test)]
mod tests {

    mod game {
        use crate::{Guess, Guesser};

        impl<F> Guesser for F where F: Fn(&[Guess]) -> String {
            fn guess(&mut self, history: &[Guess]) -> String {
                (*self)(history)
            }
        }

        #[test]
        fn play() {
            let w = crate::Wordle::new();
            let guesser = guesser!(|_history| { "moved".to_string() });
            //fn guess(_: &[Guess]) -> String {
                //"moved".to_string()
            //}
            w.play("moved", |_history: &[Guess]| "moved".to_string());
        }
    }

    mod compute {
        use crate::Correctness;

        macro_rules! mask {
            (C) => {Correctness::Correct};
            (M) => {Correctness::Misplaced};
            (W) => {Correctness::Wrong};
            ($($c:tt)+) => {[
                $(mask!($c)),+
            ]}
        }

        #[test]
        fn basic() {
            assert_eq!(Correctness::compute("abcde", "abcde"), mask!(C C C C C));
        }

    }

}
