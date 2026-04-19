pub fn byte_len(input: &[u8]) -> usize {
    input.len()
}

pub fn greeting(name: &str) -> String {
    let name = name.trim();

    if name.is_empty() {
        "Hello from Rust".to_owned()
    } else {
        format!("Hello, {name} from Rust")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn byte_len_counts_bytes_not_chars() {
        assert_eq!(byte_len(b"hello"), 5);
        assert_eq!(byte_len("你好".as_bytes()), 6);
    }

    #[test]
    fn greeting_trims_names_and_handles_empty_input() {
        assert_eq!(greeting(" Sean "), "Hello, Sean from Rust");
        assert_eq!(greeting("  "), "Hello from Rust");
    }
}
