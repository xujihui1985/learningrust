use failure_derive::{Fail};
use std::fs::File;
use std::fs::OpenOptions;
use std::io;

fn main() {
    println!("Hello, world!");
}

#[derive(Debug, Fail)]
pub enum DocumentServiceError {
  #[fail(display="aaaaaaaaaa")]
  RateLimitExceed,
  #[fail(display="I/O error: {}", _0)]
  IO(io::Error),
}

pub type Result<T> = std::result::Result<T, DocumentServiceError>;

impl From<io::Error> for DocumentServiceError {
    fn from(io: io::Error) -> Self {
      DocumentServiceError::IO(io)
    }
  
}

const MAX_DOCS_CREATED_PER_MINUTES: u8 = 100;

fn num_documents_created_in_last_minute() -> u8 {
    2
}

fn create_document(filename: &str) -> Result<File> {
    if num_documents_created_in_last_minute() > MAX_DOCS_CREATED_PER_MINUTES {
        return Err(DocumentServiceError::RateLimitExceed);
    }

    let file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(filename)?;
    Ok(file)
}