use std::borrow::Cow;
use std::ops::{Deref, DerefMut};
use std::path::Path;

fn main() {
    let mut s = Selector {
        elements: vec!['x', 'y', 'z'],
        current: 2,
    };
    assert_eq!(*s, 'z');
    assert!(s.is_alphabetic());
    *s = 'w';
    assert_eq!(*s, 'w');

    let s2 = Selector {
        elements: vec!["hello", "world"],
        current: 1,
    };
    show_it(&s2);

    let t2 = Token2::new("aaaa");
    let t3 = Token::new(secret_from_vault("hello"));

//    let path = Path::new("foo.txt");
}

#[derive(Debug)]
struct Selector<T> {
    elements: Vec<T>,
    current: usize,
}

impl<T> Deref for Selector<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.elements[self.current]
    }
}

impl<T> DerefMut for Selector<T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.elements[self.current]
    }
}

fn show_it(thing: &str) {
    println!("{}", thing);
}

enum Error {
    OutOfMemory,
    StackOverflow,
    MachineOnFire,
    Unfathomable,
    FileNotFound,
}

fn describe(error: &Error) -> Cow<'static, str> {
    match *error {
        Error::OutOfMemory => "out of memory".into(),
        Error::FileNotFound => {
            format!("file not found {}", "aaaa").into()
        }
        _ => {
            format!("file not found {}", "bbbbbbb").into()
        }
    }
}

struct Token {
    raw: String
}

impl Token {
    pub fn new<S>(raw: S) -> Token
        where S: Into<String>
    {
        Token { raw: raw.into() }
    }
}

fn secret_from_vault(key: &str) -> String {
    key.to_string()
}

struct Token2<'a> {
    raw: Cow<'a, str>
}

impl<'a> Token2<'a> {
    pub fn new<S>(raw: S) -> Token2<'a>
        where S: Into<Cow<'a, str>>
    {
        Token2 { raw: raw.into() }
    }
}

