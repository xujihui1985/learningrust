use std::{time, thread};

fn main() {
    for n in 1..101 {
        let mut handlers = Vec::with_capacity(n);

        let start = time::Instant::now();

        for _m in 0..n {
            let hander = thread::spawn(|| {
                let start = time::Instant::now();
                let pause = time::Duration::from_millis(20);
                while start.elapsed() < pause {
                    thread::yield_now();
                }
            });
            handlers.push(hander);
        }

        while let Some(hander) = handlers.pop() {
            hander.join();
        }

        let finish = time::Instant::now();
        println!("{}\t{:05?}", n, finish.duration_since(start));
    }
}
