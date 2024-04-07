fn main() {
    let a = {
        let v = 123;
        &v as *const _ as usize
    };
    let b = {
        let v = 123;
        &v as *const _ as usize
    };

    println!("{}, {}, {}",  a, b, a==b);
    println!("{}, {}, {}",  a, b, a==b);
}
