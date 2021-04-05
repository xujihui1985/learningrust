use std::{cell::UnsafeCell, sync::atomic::AtomicBool, sync::atomic::Ordering, thread};

const LOCKED: bool = true;
const UNLCOKED: bool = false;

struct Mutex<T> {
    locked: AtomicBool,
    v: UnsafeCell<T>,
}

unsafe impl<T> Sync for Mutex<T> where T: Send {}

impl<T> Mutex<T> {
    pub fn new(t: T) -> Self {
        Self {
            locked: AtomicBool::new(UNLCOKED),
            v: UnsafeCell::new(t),
        }
    }

    fn with_lock<R>(&self, f: impl FnOnce(&mut T) -> R) -> R {
        while self.locked.load(Ordering::Acquire) != UNLCOKED {}
        thread::yield_now();
        self.locked.store(LOCKED, Ordering::Relaxed);
        let ret = f(unsafe { &mut *self.v.get() });
        self.locked.store(UNLCOKED, Ordering::Release);
        ret
    }

    fn with_lock2<R>(&self, f: impl FnOnce(&mut T) -> R) -> R {
        // compare_exchange is very often but not always used in a loop
        // Acquire order guarantee that no operation can be reordered before this load
        // the second ordering in compare_exchange means, which order should the load have
        // if the load indicates that you shouldn't store
        while self
            .locked
            .compare_exchange_weak(UNLCOKED, LOCKED, Ordering::Acquire, Ordering::Relaxed)
            .is_err()
        {
            // MESI protocol: stay in S when locked
            while self.locked.load(Ordering::Relaxed) == LOCKED {

                thread::yield_now();
            }

            thread::yield_now();
            // in ARM, there is no CAS instruction, there are two instruction, LDREX, STREX
            // so compare_exchange is impl using a loop of LDREX and STREX, it will only fail
            // if current value is not match
            // compare_exchange_weak: on the other side, impl using LDREX and STREX is allow to spurious fail
            // if you already call it in a loop, you should use compare exchange weak
        }
        let ret = f(unsafe { &mut *self.v.get() });
        self.locked.store(UNLCOKED, Ordering::Release); 
        // when use release order here, there is 2 guarantee
        // 1. any operation before the store is not allowed to reorder after release
        // 2. whoever next takes the lock must see anything that happened before this store
        ret
    }
}

fn main() {
    let l: &'static _ = Box::leak(Box::new(Mutex::new(0)));
    let handles: Vec<_> = (0..10)
        .map(|_| {
            thread::spawn(move || {
                for _ in 0..100 {
                    l.with_lock2(|v| {
                        *v += 1;
                    });
                }
            })
        })
        .collect();

    for handle in handles {
        handle.join().unwrap();
    }

    let res = l.with_lock2(|v| *v);
    println!("result is {}", res);
}

#[cfg(test)]
mod tests {
    use std::sync::atomic::{AtomicUsize, Ordering};

    #[test]
    fn too_relaxed() {
        // when you have multiple threads executing concurrently, there is
        // no promise what can read from something another thread wrote under
        // ordering relaxed
        let x: &'static _ = Box::leak(Box::new(AtomicUsize::new(0)));
        let y: &'static _ = Box::leak(Box::new(AtomicUsize::new(0)));

        let t1 = std::thread::spawn(move || {
            let r1 = y.load(Ordering::Relaxed);
            x.store(r1, Ordering::Relaxed);
            r1
        });

        let t2 = std::thread::spawn(move || {
            let r2 = x.load(Ordering::Relaxed); // with relaxed, x is allowed to see any value ever stored to X, includes 42
            // the complier is totally allowed to reorder these two operations, and cpu is allowd to reorder the execution
            y.store(42, Ordering::Relaxed);
            r2
        });

        let r1 = t1.join().unwrap();
        let r2 = t2.join().unwrap();
    }
}
j