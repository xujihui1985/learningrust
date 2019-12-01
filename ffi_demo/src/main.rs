#[link(name="doubler")]
extern "C" {
    fn doubler(x: u32) -> u32;
}

fn r_doubler(x: u32) -> u32 {
    unsafe {
        doubler(x)
    }
}

fn main() {
    let res = r_doubler(123);
    println!("res is {}", res);
}
