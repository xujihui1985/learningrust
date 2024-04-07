fn main() {
    println!("Hello, world!");
}

fn sum<T>(v: &[T]) -> T
where
    T: std::ops::AddAssign + Default + Copy
{
    let mut res = Default::default();
    for i in v {
        res += *i;
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum() {
        assert_eq!(sum(&[1, 2, 3]), 6);
        assert_eq!(sum(&[1.0, 2.0, 3.0]), 6.0);
    }
}