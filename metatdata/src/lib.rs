use std::collections::HashMap;

pub use metadata_derive::MetaData;

pub trait MetaData {
    // avoid heap allocation during compile time by returning a reference
    fn author(&self) -> &str;
    fn serial_version(&self) -> usize;
    fn field_authors(&self) -> HashMap<&str, &str>;
}