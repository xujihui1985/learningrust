use quick_error::quick_error;
use quick_error::ResultExt;
use std::env;
use std::fs::File;
use std::fs::OpenOptions;
use std::io;

pub type Result<T> = std::result::Result<T, DocumentServiceError>;
const MAX_DOCS_CREATED_PER_MINUTES: u8 = 100;

pub fn num_threads() -> std::result::Result<usize, Box<dyn std::error::Error>> {
    let s = env::var("NUM_THEADS")?;
    let n: usize = s.parse()?;
    Ok(n + 1)
}

quick_error! {
  #[derive(Debug)]
  pub enum DocumentServiceError {
    RateLimitExceed {
      display("aaaaaaaaaaaaaaaaaa")
    }
    IO(filename: String, err: io::Error) {
      display("I/O error: {}", err)
      context(filename: &'a str, err: io::Error)
        -> (filename.to_string(), err)
    }
  }
}

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
        .open(filename)
        .context(filename)?;
    Ok(file)
}
