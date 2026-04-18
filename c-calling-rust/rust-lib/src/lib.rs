pub mod calculator;
pub mod ffi;

#[cfg(test)]
mod tests {
    use super::{calculator, ffi};

    #[test]
    fn it_works() {
        let result = calculator::add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn exported_add_uses_same_implementation() {
        let result = ffi::rust_lib_add(20, 22);
        assert_eq!(result, 42);
    }
}
