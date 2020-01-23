use std::io::prelude::*;
use std::fs::File;
use chrono::*;

fn main() {
    let mut f = File::create("hello.txt").unwrap();
    let local = Local::now();
    let time_str = local.format("%Y").to_string();
    let bytes = time_str.as_bytes();
    match f.write_all(bytes) {
        Err(e) => println!("failed to write file {:?}", e),
        _ => println!("success")
    }

}
