use std::os::raw::c_char;
use std::os::raw::c_void;
use std::mem;

#[repr(C)]
pub struct NativeVec {
    vec: *mut u32,
    vec_len: usize,
    vec_cap: usize,
}

#[no_mangle]
pub extern "C" fn new_vec() -> *mut NativeVec {
    let v = vec![1_u32,2,3];
    // prevent running v's destructor so we are in complete control of the allocation
    // this is important
    let mut v = mem::ManuallyDrop::new(v);
    let nv = Box::new(NativeVec {
        vec: v.as_mut_ptr(),
        vec_len: v.len(),
        vec_cap: v.capacity(),
    });
    Box::into_raw(nv)
}

#[no_mangle]
pub extern "C" fn mutate_vec(v: *mut NativeVec) {
     unsafe {
        let v = Box::from_raw(v);
        let v2 = Vec::from_raw_parts(v.vec, v.vec_len, v.vec_cap);
        println!("len {:?}", v2.len());
        println!("cap {:?}", v2.capacity());
        println!("first {:?}", v2[0]);
        println!("second {:?}", v2[1]);
        println!("third {:?}", v2[2]);
        drop(v2);
    };
}



#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
