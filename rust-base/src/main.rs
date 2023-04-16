#![allow(unused)]

use crate::prelude::*;
use std::fs::read_dir;

mod error;
mod prelude; 
mod utils;

fn main() -> Result<()> {

    for entry in read_dir("./")?.filter_map(|e| e.ok()) {
        let entry: String = W(&entry).try_into()?;
        // let entry = entry
        //     .path()
        //     .to_str()
        //     .map(String::from)
        //     .ok_or_else(|| {
        //         Error::Generic(f!("invalid path {entry:?}"))
        //     })?;
        println!("{entry:?}");
    }

    Ok(())
}
