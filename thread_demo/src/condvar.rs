use std::{
    sync::{Arc, Condvar, Mutex},
    thread,
};

fn main() {
    let total_readers = 5;
    let mutcond = Arc::new((Mutex::new((false, 0)), Condvar::new()));

    let mut reader_jhs = Vec::with_capacity(total_readers);

    for _ in 0..total_readers {
        reader_jhs.push(thread::spawn({
            let mutcond = Arc::clone(&mutcond);
            move || {
                let mut total_zeros = 0;
                let mut total_wakes = 0;
                let (mtx, cnd) = &*mutcond;

                while total_zeros < 100 {
                    let mut guard = mtx.lock().unwrap();
                    while !guard.0 {
                        guard = cnd.wait(guard).unwrap();
                    }
                    guard.0 = false;

                    println!("inc wakes {}", total_wakes);
                    total_wakes += 1;
                    if guard.1 == 0 {
                        total_zeros += 1;
                    }
                }
                total_wakes
            }
        }))
    }

    thread::Builder::new().name("writer".into()).spawn({
        let mutcond = Arc::clone(&mutcond);
        move || {
            let (mtx, cnd) = &*mutcond;
            loop {
              let mut guard = mtx.lock().unwrap();
              guard.1 = (guard.1 as u16).wrapping_add(1);
              guard.0 = true;
              cnd.notify_all();
            }
        }
    });

    for jh in reader_jhs {
      println!("{:?}", jh.join().unwrap());
    }
}
