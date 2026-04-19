#[derive(Debug, Copy, Clone, PartialEq)]
pub struct I32Stats {
    pub len: usize,
    pub sum: i64,
    pub min: i32,
    pub max: i32,
    pub mean: f64,
}

pub fn sum_i32(values: &[i32]) -> i64 {
    values.iter().map(|value| i64::from(*value)).sum()
}

pub fn analyze_i32(values: &[i32]) -> Option<I32Stats> {
    let (&first, rest) = values.split_first()?;

    let mut sum = i64::from(first);
    let mut min = first;
    let mut max = first;

    for &value in rest {
        sum += i64::from(value);
        min = min.min(value);
        max = max.max(value);
    }

    Some(I32Stats {
        len: values.len(),
        sum,
        min,
        max,
        mean: sum as f64 / values.len() as f64,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sum_i32_accumulates_as_i64() {
        assert_eq!(sum_i32(&[1, 2, 3, 4, 5]), 15);
    }

    #[test]
    fn analyze_i32_calculates_aggregates() {
        let values = [-2, 7, 3, 0];

        let stats = analyze_i32(&values).expect("non-empty values have stats");

        assert_eq!(stats.len, 4);
        assert_eq!(stats.sum, 8);
        assert_eq!(stats.min, -2);
        assert_eq!(stats.max, 7);
        assert_eq!(stats.mean, 2.0);
    }

    #[test]
    fn analyze_i32_rejects_empty_input() {
        assert_eq!(analyze_i32(&[]), None);
    }
}
