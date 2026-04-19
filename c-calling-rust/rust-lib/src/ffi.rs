use crate::{calculator, geometry, numbers, text};
use std::{
    ffi::{CStr, CString},
    os::raw::c_char,
    ptr, slice,
};

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct RustLibPoint {
    pub x: f64,
    pub y: f64,
}

impl From<RustLibPoint> for geometry::Point {
    fn from(point: RustLibPoint) -> Self {
        Self::new(point.x, point.y)
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct RustLibI32Stats {
    pub len: usize,
    pub sum: i64,
    pub min: i32,
    pub max: i32,
    pub mean: f64,
}

impl From<numbers::I32Stats> for RustLibI32Stats {
    fn from(stats: numbers::I32Stats) -> Self {
        Self {
            len: stats.len,
            sum: stats.sum,
            min: stats.min,
            max: stats.max,
            mean: stats.mean,
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn rust_lib_add(left: u64, right: u64) -> u64 {
    calculator::add(left, right)
}

/// Returns the byte length of a NUL-terminated C string.
///
/// # Safety
///
/// `input` must be null or point to a valid NUL-terminated C string.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rust_lib_c_string_len(input: *const c_char) -> usize {
    if input.is_null() {
        return 0;
    }

    let input = unsafe { CStr::from_ptr(input) }.to_bytes();
    text::byte_len(input)
}

/// Allocates a greeting string owned by Rust.
///
/// # Safety
///
/// `name` must be null or point to a valid NUL-terminated C string. A non-null
/// return value must be released exactly once with `rust_lib_string_free`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rust_lib_greeting_new(name: *const c_char) -> *mut c_char {
    if name.is_null() {
        return ptr::null_mut();
    }

    let name = unsafe { CStr::from_ptr(name) }.to_string_lossy();
    let greeting = text::greeting(&name);

    CString::new(greeting)
        .expect("generated greeting does not contain interior NUL bytes")
        .into_raw()
}

/// Releases a string allocated by Rust.
///
/// # Safety
///
/// `value` must be null or a pointer returned by `rust_lib_greeting_new` that
/// has not already been freed.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rust_lib_string_free(value: *mut c_char) {
    if value.is_null() {
        return;
    }

    unsafe {
        drop(CString::from_raw(value));
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn rust_lib_point_distance_from_origin(point: RustLibPoint) -> f64 {
    geometry::Point::from(point).distance_from_origin()
}

/// Sums a borrowed C array of `int32_t` values.
///
/// # Safety
///
/// If `len` is non-zero, `values` must point to at least `len` initialized
/// `int32_t` elements.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rust_lib_sum_i32(values: *const i32, len: usize) -> i64 {
    if len == 0 || values.is_null() {
        return 0;
    }

    let values = unsafe { slice::from_raw_parts(values, len) };
    numbers::sum_i32(values)
}

/// Calculates aggregate statistics for a borrowed C array of `int32_t` values.
///
/// # Safety
///
/// If `len` is non-zero, `values` must point to at least `len` initialized
/// `int32_t` elements. `out_stats` must be null or point to writable memory for
/// one `RustLibI32Stats`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rust_lib_analyze_i32(
    values: *const i32,
    len: usize,
    out_stats: *mut RustLibI32Stats,
) -> bool {
    if len == 0 || values.is_null() || out_stats.is_null() {
        return false;
    }

    let values = unsafe { slice::from_raw_parts(values, len) };
    let Some(stats) = numbers::analyze_i32(values) else {
        return false;
    };

    unsafe {
        out_stats.write(stats.into());
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{
        ffi::{CStr, CString},
        ptr,
    };

    #[test]
    fn exported_add_uses_same_implementation() {
        let result = rust_lib_add(20, 22);

        assert_eq!(result, 42);
    }

    #[test]
    fn exported_string_len_counts_c_string_bytes() {
        let input = CString::new("hello").expect("test string has no interior NUL");

        let result = unsafe { rust_lib_c_string_len(input.as_ptr()) };

        assert_eq!(result, 5);
    }

    #[test]
    fn exported_string_len_handles_null() {
        let result = unsafe { rust_lib_c_string_len(ptr::null()) };

        assert_eq!(result, 0);
    }

    #[test]
    fn exported_greeting_allocates_c_string() {
        let name = CString::new("Sean").expect("test string has no interior NUL");

        let greeting = unsafe { rust_lib_greeting_new(name.as_ptr()) };

        assert!(!greeting.is_null());

        let greeting_text = unsafe { CStr::from_ptr(greeting) }
            .to_str()
            .expect("greeting is valid UTF-8");

        assert_eq!(greeting_text, "Hello, Sean from Rust");

        unsafe {
            rust_lib_string_free(greeting);
        }
    }

    #[test]
    fn exported_point_distance_accepts_c_layout_struct() {
        let point = RustLibPoint { x: 3.0, y: 4.0 };

        let result = rust_lib_point_distance_from_origin(point);

        assert_eq!(result, 5.0);
    }

    #[test]
    fn exported_array_sum_reads_c_array() {
        let values = [1, 2, 3, 4, 5];

        let result = unsafe { rust_lib_sum_i32(values.as_ptr(), values.len()) };

        assert_eq!(result, 15);
    }

    #[test]
    fn exported_array_sum_handles_empty_or_null() {
        let empty_result = unsafe { rust_lib_sum_i32(ptr::null(), 0) };
        let null_result = unsafe { rust_lib_sum_i32(ptr::null(), 5) };

        assert_eq!(empty_result, 0);
        assert_eq!(null_result, 0);
    }

    #[test]
    fn exported_array_analysis_writes_output_stats() {
        let values = [-2, 7, 3, 0];
        let mut stats = RustLibI32Stats {
            len: 0,
            sum: 0,
            min: 0,
            max: 0,
            mean: 0.0,
        };

        let ok = unsafe { rust_lib_analyze_i32(values.as_ptr(), values.len(), &mut stats) };

        assert!(ok);
        assert_eq!(stats.len, 4);
        assert_eq!(stats.sum, 8);
        assert_eq!(stats.min, -2);
        assert_eq!(stats.max, 7);
        assert_eq!(stats.mean, 2.0);
    }

    #[test]
    fn exported_array_analysis_rejects_invalid_inputs() {
        let values = [1, 2, 3];
        let mut stats = RustLibI32Stats {
            len: 0,
            sum: 0,
            min: 0,
            max: 0,
            mean: 0.0,
        };

        let empty_result = unsafe { rust_lib_analyze_i32(values.as_ptr(), 0, &mut stats) };
        let null_values_result =
            unsafe { rust_lib_analyze_i32(ptr::null(), values.len(), &mut stats) };
        let null_out_result =
            unsafe { rust_lib_analyze_i32(values.as_ptr(), values.len(), ptr::null_mut()) };

        assert!(!empty_result);
        assert!(!null_values_result);
        assert!(!null_out_result);
    }
}
