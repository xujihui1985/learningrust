mod custom_error;
use std::io;
use std::result;
use std::error::Error;

pub type Result<T> = result::Result<T, Error>;
type GenError = Box<std::error::Error>;
type GenResult<T> = result::Result<T, GenError>;

fn compile_project() -> GenResult<()> {
    let io_error = io::Error::new(io::ErrorKind::Other, "timeout");
    return Err(GenError::from(io_error));
}


#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test_io_error() {
        loop {
            match compile_project() {
                Ok(()) => println!("compile success"),
                Err(err) => {
                    if let Some(io_error) = err.downcast_ref::<io::Error>() {
                        println!("io error");
                        break;
                    }
                    panic!("unknown error");
                }
            }
        }

    }

    #[test]
    fn custom_error() {

    }
}


