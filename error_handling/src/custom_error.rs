use std::error::Error;
use std::env;
use std::fs::File;
use std::fs::OpenOptions;
use std::io;
use std::fmt;

pub fn num_threads() -> std::result::Result<usize, Box<dyn std::error::Error>> {
    let s = env::var("NUM_THEADS")?;
    let n: usize = s.parse()?;
    Ok(n + 1)
}

// using box error
// cons
// can't inspect the error type in code
// can't decide to handle different errors differently

// custom error type
// called by ? to convert between error types

#[derive(Debug)]
pub enum DocumentServiceError {
  RateLimitExceed,
  IO(io::Error),
}

pub type Result<T> = std::result::Result<T, DocumentServiceError>;

impl From<io::Error> for DocumentServiceError {
    fn from(io: io::Error) -> Self {
      DocumentServiceError::IO(io)
    }
  
}

impl fmt::Display for DocumentServiceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      match self {
        DocumentServiceError::RateLimitExceed => write!(f, "rate limit error"),
        DocumentServiceError::IO(io) => write!(f, "I/O error: {}", io),
      }
    }

}

impl Error for DocumentServiceError { }

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
