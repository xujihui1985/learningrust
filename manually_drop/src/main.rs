use std::{mem::ManuallyDrop, fs::File, os::unix::prelude::{AsRawFd, IntoRawFd, FromRawFd, RawFd}, io::Read};

fn main() {
    // f.as_raw_fd()
    let fd = get_fd();
    let mut f = unsafe {File::from_raw_fd(fd)};
    let mut s = String::new();
    f.read_to_string(&mut s).unwrap();
    println!("result is {}", s);


    // let mut v = ManuallyDrop::new(vec![65, 122]);
    // {
    //     let s = unsafe { String::from_raw_parts(v.as_mut_ptr(), v.len(), v.capacity()) };
    //     assert_eq!(s, "Az");
    // }

    // println!("111111111111");
    // let _ = ManuallyDrop::into_inner(v);
    // println!("222222222222");
}

fn get_fd() -> RawFd {
    let f = File::open("/Users/sean/code/github.com/xujihui1985/learningrust/manually_drop/cargo.toml").unwrap();
    // let f = ManuallyDrop::new(f);
    // f.as_raw_fd()
    let fd = f.as_raw_fd();
    fd
}