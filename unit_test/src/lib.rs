//! `bucketize` is a crate for slotting numerical values into buckets.
//! to do this, create a `Bucketizer` and add your buckets to it,
//! # Example
//! ```
//! ```

/// A bucketizer holds the list of buckets you want to slot values into
/// # Example
/// ```
/// use mybuck::Bucketizer;
/// let b = Bucketizer::new()
///     .bucket(Some(1.0), Some(2.0), 1.0);
/// assert_eq!(b.bucketizer(2.0), None);
/// ```
#[derive(Clone, Debug, PartialEq)]
pub struct Bucketizer {
    buckets: Vec<Bucket>
}

type Bucket = (Option<f64>, Option<f64>, f64);

impl Bucketizer {
    pub fn new() -> Self {
        Bucketizer {
            buckets: vec![]
        }
    }

    pub fn bucket(mut self, min: Option<f64>, max: Option<f64>, value: f64) -> Self {
        self.buckets.push((min, max, value));
        self
    }

    /// Returns the value of the bucket that the input falls into
    /// 
    pub fn bucketizer(&self, input: f64) -> Option<f64> {
        for b in &self.buckets {
            match *b {
                (None, None, val) => return Some(val),
                (Some(min), None, val) => {
                    if input >= min {
                        return Some(val);
                    }
                },
                (None, Some(max), val) => {
                    if input < max {
                        return Some(val);
                    }
                },
                (Some(min), Some(max), val) => {
                    if input >= min && input < max {
                        return Some(val);
                    }
                }
            }
        }
        return None;
    }
}

macro_rules! say_hello {
    () => {
        println!("hello world");
    };

    ($name:expr) => {
        println!("hello world {}", $name);
    };

    ($($x:expr),*) => {
        $(
            println!("hello world {}", $x);
        )*
    };
}

#[cfg(test)]
mod tests {

    use std::{collections::BTreeMap, iter::Map};

    use super::Bucketizer;

    #[test]
    fn it_works() {
        let b = Bucketizer::new()
            .bucket(Some(0.0), Some(1.0), 0.5);
        assert_eq!(b.bucketizer(0.1), Some(0.5));
    }

    #[test]
    fn single_bucket_end_values() {
        let b = Bucketizer::new()
            .bucket(Some(0.0), Some(1.0), 0.5);
    }

    pub fn divide(a: i32, b: i32) -> i32 {
        if b == 0 {
            panic!("panic");
        }
        a/b
    }

    #[test]
    #[should_panic(expected = "panic")]
    fn test_bucket_panic() {
        divide(1, 0);
    }

    #[test]
    fn test_hello_macro() {
        say_hello!();
        say_hello!(1 + 1);
        say_hello!(1,2,3);
    }

    #[test]
    fn test_tree_map() {
        #[derive(Debug)]
        struct Page {
            id: u32,
            dirty: bool,
        }

        impl Page {
            fn is_dirty(&self) -> bool {
                self.dirty
            }

            fn set_dirty(&mut self) {
                self.dirty = true;
            }
        }
        let mut t = BTreeMap::new();
        t.insert(1, Page{id: 1, dirty: false});
        t.insert(2, Page{id: 2, dirty: false});
        t.insert(9, Page{id: 3, dirty: false});
        t.insert(128, Page{id: 3, dirty: false});

        let res = t.range(1..10);
        for (&k, v) in res {
            println!("key is {}", k);
        }

        for (&k, v) in t.range_mut(2..) {
            v.id = 5;
            v.set_dirty();
        }

        let res = t.range(1..10);
        for (&k, v) in res {
            println!("key is {} value is {:?}", k, v);
        }


    }

}
