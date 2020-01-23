use std::fmt::Display;

fn show_all(v: Vec<&dyn Display>) {
    for i in v {
        println!("item is {}", i);
    }
}

fn main() {
    let v = vec![&1 as &dyn Display, &"hello" as &dyn Display];
    show_all(v);
}
