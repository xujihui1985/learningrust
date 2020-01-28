use std::fs::File;
use std::io;
use std::io::{Read, Write};

pub fn create_file() -> std::io::Result<()> {
    let mut s = String::new();
    let mut f = File::open("data/from.md")?;
    f.read_to_string(&mut s)?;
    println!("content {}", s);
    let mut t = File::create("data/to.md")?;
//    io::copy(&mut f, &mut t)?;
    t.write_all(&s.into_bytes())?;
    Ok(())
}