#[macro_use]
extern crate error_chain;
//use error_chain::error_chain;
//use error_chain::bail;
// use error_chain::error_chain;
use std::error::Error;
use std::fs::File;
use std::fs::OpenOptions;
use std::io;
use std::fmt;

fn main() {
    println!("Hello, world!");
}

pub mod errors {
    error_chain! {
        errors {
            RateLimitExceed {
                display("aaaaaaa")
            }
        }
        foreign_links {
            IO(::std::io::Error);
        }
    }
}

use errors::*;

const MAX_DOCS_CREATED_PER_MINUTES: u8 = 100;

fn num_documents_created_in_last_minute() -> u8 {
    2
}

fn create_document(filename: &str) -> Result<File> {
    if num_documents_created_in_last_minute() > MAX_DOCS_CREATED_PER_MINUTES {
        bail!(ErrorKind::RateLimitExceed);
    }

    let file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(filename)?;
    Ok(file)
}