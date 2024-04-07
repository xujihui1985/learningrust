fn main() {
    let a = 42.42_f32;

    let ftype: u32 = unsafe {
        std::mem::transmute(a)
    };

    println!("{}", ftype);
    println!("{:032b}", ftype); // left-pad with 32 zeros

    let b: f32 = unsafe {
        std::mem::transmute(ftype)
    };

    println!("{}", b);
}
