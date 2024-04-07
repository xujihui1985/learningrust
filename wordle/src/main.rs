use std::fs::OpenOptions;
use std::os::unix::fs::FileExt;

const GAMES: &str = include_str!("../answers.txt");

fn main() {
    //for answer in GAMES.split_whitespace() {
    //let guesser = wordle::algorithms::Naive::new();
    //   wordle::play(answer, guesser);
    //}
    //
    let outer = String::from("hello");
    //call_fn_pointer(|s| {
    //println!("{}", outer.len());
    //s.to_string()
    //});
    // call_fn_trait(|s| {
    //     println!("{}", outer.len());
    //     outer
    // });
    with_foo(|s: &str| s.to_string());

    let mut s = "hello";
    let mutstr = MutStr { s: &mut s };
    *mutstr.s = "world";
    // println!("{}", s);
}

struct MutStr<'a> {
    s: &'a mut &'a str,
}

fn call_fn_pointer(f: fn(&str) -> String) {
    let res = f("fn pointer");
    println!("{}", res);
}

fn call_fn_trait<T: Fn(&str) -> String>(f: T) {
    let res = f("fn trait");
    println!("{}", res);
}

trait Foo {
    fn call(&mut self, msg: &str) -> String;
}

//impl Foo for fn(&str) -> String {
//fn call(&mut self, msg: &str) -> String {
//(*self)(msg)
//}
//}

impl<F> Foo for F
where
    F: Fn(&str) -> String,
{
    fn call(&mut self, msg: &str) -> String {
        (*self)(msg)
    }
}

fn with_foo<F: Foo>(mut foo: F) -> Option<String> {
    let res = foo.call("hello");
    Some(res)
}

struct Thing {
    field: String,
}

// fn copy_fn1(t: &Thing) -> String {
//     let tmp = *t;
//     tmp.field.clone()
// }

// fn copy_fn2(t: &Thing) -> Thing {
//     *t
// }
