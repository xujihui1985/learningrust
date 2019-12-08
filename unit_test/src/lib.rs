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

#[cfg(test)]
mod tests {

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
}
