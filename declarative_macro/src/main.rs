mod avec;

avec::avec!{
    u32 as name;
}

avec::max_impl!(i32);

fn main() {
    let v:Vec<u32> = avec::avec![];
    let v2 = avec::avec![10, 12,];
    println!("len {}", v2.len());
    let a:i32 = 123;
    max_val(a);
}

fn max_val<T: avec::MaxValue>(x: T) {
    T::max_value();
}
