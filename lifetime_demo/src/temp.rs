use std::{sync::Mutex, fs::File, io::Write};

struct User {
    name: String,
    jobs: Vec<String>,
}

impl User {
    fn set_name(&mut self, name: String) {
        self.name = name;
    }
}

fn main() {
    String::from('🦀').as_bytes().contains(&0x80);
    let_fn();

    let mut user = User {
        name: String::from("foo"),
        jobs: vec![String::from("bar"), String::from("baz")],
    };
    let jobs = user.jobs;
    // user.set_name(String::from("qux"));



    // let filename = "hello.txt";
    // let file = File::create(filename).unwrap();
    // let mut writer = Writer { file: &file };
    // writer.write("hello world");

    // let mut writer = {
    //     let filename = "hello.txt";
    //     let file = File::create(filename).unwrap();
    //     Writer::new(&file)
    // };
    // writer.write("hello world");

    let mut writer = {
        let filename = "hello.txt";
        // let file = File::create(filename).unwrap();
        Writer { file: &File::create(filename).unwrap() }
    };
    writer.write("hello world");
}

fn let_fn() {
    let s = f(&String::from("hello"));
    let a = &String::from("world");
    g(a);
    // g(s);
}

fn f(s: &str) -> &str {
    s
}

fn g(s: &str){}

fn example(m: &Mutex<String>) {
    if m.lock().unwrap().is_empty() {
        println!("the string is empty!");
    }
}

pub struct Writer<'a> {
    pub file: &'a File
}

impl <'a> Writer<'a> {
    pub fn write(&mut self, data: &str) {
      self.file.write_all(data.as_bytes());
    }

    pub fn new(file: &'a File) -> Self {
        Writer { file }
    }
}
