use std::collections::HashMap;
use std::fmt;
use std::fmt::Display;
use std::fs::File;
use std::io::ErrorKind;

mod client;
mod network;

struct List(Vec<i32>);

struct User<'a> {
    username: &'a str,
    email: &'a str,
    active: bool,
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let vec = &self.0;
        write!(f, "[")?;

        for v in vec {
            write!(f, "{},", v).unwrap();
        }

        write!(f, "]")
    }
}

fn main() {
    let v = List(vec![1, 2, 3]);
    println!("{}", v);

    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("the length of {} is {}", s1, len);
//    let first = first_word(&s1);
//    println!("the length of {} is {}", s1, len);

    let v2 = vec![("hello".to_string(), 5)];
    let names = get_names(v2);
    assert_eq!(names, ["hello"]);

    client::connect();
    network::server::connect();

    let row = vec![
        client::SpreadsheetCell::Int(10),
        client::SpreadsheetCell::Float(10.1),
        client::SpreadsheetCell::Text(String::from("hello")),
    ];

    let mut scores = HashMap::new();
    let hello = String::from("hello");
    scores.insert(hello, 10); // hello is moved into hashmap, it is invalid since here
    // println!("{}", hello); // this will not compile
    scores.insert(String::from("world"), 10);

    let k = "hello".to_string();
    let res = scores.get(&k);

    let f = File::open("hello.txt");
    let f = match f {
        Ok(file) => file,
        Err(ref error) if error.kind() == ErrorKind::NotFound => {
            match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => {
                    panic!("failed to create file ${:?}", e)
                }
            }
        }
        Err(error) => {
            panic!("failed to open")
        }
    };
}

fn calculate_length(s: &str) -> usize {
    s.len()
}

fn dangle() -> &'static str {
    let s = "hello";
    &s
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

fn get_names(v: Vec<(String, usize)>) -> Vec<String> {
    let res = v.into_iter()
        .map(|(name, _score)| name)
        .collect();
    res
}

fn longest_with_an_annotation<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
    where T: Display
{
    println!("Anno {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
