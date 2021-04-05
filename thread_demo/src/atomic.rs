use std::{sync::atomic::Ordering, thread};

struct Leak;

impl Drop for Leak {
    fn drop(&mut self) {
        println!("leak dropped");
    }
}

fn main() {
    {
        let x = Box::new(Leak);
        // let static_ref = Box::leak(x);
    }
    println!("after leak");
}
