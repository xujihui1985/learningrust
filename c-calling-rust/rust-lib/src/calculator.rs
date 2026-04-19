pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_returns_sum() {
        assert_eq!(add(2, 2), 4);
    }
}
