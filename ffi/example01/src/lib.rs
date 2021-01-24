use std::slice;
use std::os::raw::c_int;

#[no_mangle]
pub extern "C" fn sum_of_even(n: *const c_int, len: c_int) -> i32 {
    let numbers = unsafe {
        assert!(!n.is_null());
        slice::from_raw_parts(n, len as usize)
    };

    numbers
        .into_iter()
        .filter(|&v| v % 2 == 0)
        .sum()
}
