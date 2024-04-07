use std::sync::atomic::{AtomicBool, Ordering};

const ATOMIC_FALSE: AtomicBool = AtomicBool::new(false);
static READY: [AtomicBool; 10] = [ATOMIC_FALSE; 10];

fn main() {
    let a = READY[0].load(Ordering::Relaxed);
    let b: [bool; 20] = std::array::from_fn(|i| READY[i].load(Ordering::Relaxed));
    for i in b.iter() {
        println!("{}", i);
    }
}
