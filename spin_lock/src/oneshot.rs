use std::sync::atomic::{AtomicU8, Ordering};
use std::{cell::UnsafeCell, mem::MaybeUninit, sync::atomic::AtomicBool};

const EMPTY: u8 = 0;
const WAITING: u8 = 1;
const READY: u8 = 2;
const READING: u8 = 3;

pub struct Channel<T> {
    // since our atomic boolean ready property already tells us whether there is
    // a message or not. we can use a MaybeUninit which is essentially the bare bones
    // unsafe version of Option<T>, it requires its user to manually keep track of whether
    // it has been initialized o not
    message: UnsafeCell<MaybeUninit<T>>,
    state: AtomicU8,
}

unsafe impl<T> Sync for Channel<T> where T: Send {}

impl<T> Channel<T> {
    pub const fn new() -> Self {
        Self {
            state: AtomicU8::new(EMPTY),
            message: UnsafeCell::new(MaybeUninit::uninit()),
        }
    }

    // Safety: only call this once
    pub fn send(&self, message: T) {
        //let m = &mut *self.message.get();
        //m.write(message);
        // swap stores a value into the bool, returning the previous value
        //
        //if self.in_use.swap(true, Ordering::Acquire) {
        //panic!("can't send more than on message");
        //}
        if self
            .state
            .compare_exchange(EMPTY, WAITING, Ordering::Relaxed, Ordering::Relaxed)
            .is_err()
        {
            panic!("can't send more than on message");
        }
        unsafe {
            (*self.message.get()).write(message);
        }
        self.state.store(READY, Ordering::Release);
    }

    pub fn is_ready(&self) -> bool {
        self.state.load(Ordering::Relaxed) == READY
    }

    // Safety: only call this once, and only after is_ready() returns true
    pub fn receive(&self) -> T {
        // use swap to change the ready to false after calling receive, to make sure
        // receive can only be called once
        if self
            .state
            .compare_exchange(READY, READING, Ordering::Acquire, Ordering::Relaxed)
            .is_err()
        {
            panic!("message is not available");
        }
        unsafe { (*self.message.get()).assume_init_read() }
    }
}

impl<T> Drop for Channel<T> {
    fn drop(&mut self) {
        // because an object can only be dropped if it is fully owned by whichever
        // thread is dropping it, with no outstanding borrows. this means we can
        // use the AtomicBool::get_mut method, which takes an exclusive reference (&mut self)
        // proving that atomic access is unnecessary.
        if *self.state.get_mut() == READY {
            unsafe {
                self.message.get_mut().assume_init_drop();
            }
        }
    }
}
