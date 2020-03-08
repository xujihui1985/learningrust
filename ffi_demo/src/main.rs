use std::os::raw::c_char;
use std::os::raw::c_void;

#[link(name="doubler")]
extern "C" {
    fn doubler(x: u32) -> u32;
}

fn r_doubler(x: u32) -> u32 {
    unsafe {
        doubler(x)
    }
}

#[repr(C)]
pub struct NativeStruct {
    vec: *const u8,
    vec_len: usize,
    name: *const c_char,
}

#[no_mangle]
pub extern "C" fn rust_func() -> i32 {
    123
}

#[no_mangle]
pub extern "C" fn vec_free(ptr: *const c_void, len: usize, cap: usize) {
    let ptr = ptr as *mut u32;
    unsafe {
       // let _ = Vec::from_raw_parts(ptr, len, cap);
        let v = std::slice::from_raw_parts(ptr, len).to_owned();
    }

}

#[no_mangle]
pub extern "C" fn use_vec(ptr: *const c_void, len: usize, cap: usize) {
    let ptr = ptr as *mut u32;
    unsafe {
       let v = Vec::from_raw_parts(ptr, len, cap);
       println!("vec is {:?}", v);
    }
}

#[repr(C)]
pub struct NativeVec {
    vec: *const u32,
    vec_len: usize,
    vec_cap: usize,
}

#[no_mangle]
pub extern "C" fn get_vec() -> *mut NativeVec {
    let v = vec![1_u32,2,3];
    let nv = Box::new(NativeVec {
        vec: v.as_ptr(),
        vec_len: v.len(),
        vec_cap: v.capacity(),
    });
    Box::into_raw(nv)
}


fn main() {
    let res = r_doubler(123);
    println!("res is {}", res);
}

fn move_ownership() {
    let v = vec![0_u8];
    let s = NativeStruct { 
        vec: v.as_ptr(),
        vec_len: v.len(),
        name: "aaaa".as_ptr() as *const c_char,
    } ;
    let ptr = Box::into_raw(Box::new(s));
    // free memory
//    let _ = Box::from_raw(ptr);

 //   let ns = &s as *const NativeStruct;
}

