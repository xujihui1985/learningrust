use crate::calculator;

#[unsafe(no_mangle)]
pub extern "C" fn rust_lib_add(left: u64, right: u64) -> u64 {
    calculator::add(left, right)
}
