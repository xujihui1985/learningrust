
// reexport Error type
pub use crate::error::Error;
pub use std::format as f;

pub type Result<T> = core::result::Result<T, Error>;

// generic wrapper tuple struct for newtype pattern
pub struct W<T>(pub T);

