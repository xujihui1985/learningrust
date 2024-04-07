use std::cell::RefCell;

thread_local! {
    static COUNTER: RefCell<u32> = RefCell::new(123);
}

fn main() {
    COUNTER.with(|c| {
        *c.borrow_mut() += 10;
    });

    let h = std::thread::spawn(|| {
        COUNTER.with(|c| {
            *c.borrow_mut() += 100;
        });

        COUNTER.with(|c| println!("inside thread {}", c.borrow()));
    });
    h.join().unwrap();

    COUNTER.with(|c| println!("{}", c.borrow()));
}
