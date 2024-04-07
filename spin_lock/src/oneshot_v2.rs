use std::marker::PhantomData;
use std::sync::Arc;
use std::thread::{Thread, self};
use std::{cell::UnsafeCell, mem::MaybeUninit};
use std::sync::atomic::{Ordering, AtomicU8, AtomicBool};


pub struct Sender<'a, T> {
    channel: &'a Channel<T>,
    // represents a handle to a thread
    receiving_thread: Thread,
}

pub struct Receiver<'a, T> {
    channel: &'a Channel<T>,
    no_send: PhantomData<*const ()>, // make Receiver not sendable
}

pub struct Channel<T> {
    message: UnsafeCell<MaybeUninit<T>>,
    ready: AtomicBool,
}

impl<T> Channel<T> {
    pub const fn new() -> Self {
        Self {
            message: UnsafeCell::new(MaybeUninit::uninit()),
            ready: AtomicBool::new(false),
        }
    }

    pub fn split<'a>(&'a mut self) -> (Sender<'a, T>, Receiver<'a, T>) {
        //By overwriting *self with a new empty channel in split(), we make sure it's
        //in the expected state when creating the Sender and Receiver states. This also invokes 
        //the Drop implementation on the old *self, which will take care of dropping a message that was previously sent but not received.
        *self = Self::new();
        (Sender {channel: self, receiving_thread: thread::current()}, Receiver{channel: self, no_send: PhantomData})
    }
}

unsafe impl<T> Sync for Channel<T> where T: Send{}

//pub fn channel<T>() -> (Sender<T>, Receiver<T>) {
    //let a = Arc::new(Channel{
        //message: UnsafeCell::new(MaybeUninit::uninit()),
        //ready: AtomicBool::new(false),
    //});
    //(Sender{channel: a.clone()}, Receiver{channel: a})
//}

impl<T> Sender<'_, T> {
    pub fn send(self, message: T) {
        unsafe { (*self.channel.message.get()).write(message); }
        self.channel.ready.store(true, Ordering::Release);
        self.receiving_thread.unpark();
    }
}

impl<T> Receiver<'_, T> {
    pub fn is_ready(&self) -> bool {
        self.channel.ready.load(Ordering::Relaxed)
    }

    pub fn receive(self) -> T {
        while !self.channel.ready.swap(false, Ordering::Acquire) {
            thread::park();
        }
        unsafe { (*self.channel.message.get()).assume_init_read() }
    }
}


impl<T> Drop for Channel<T> {
    fn drop(&mut self) {
        if *self.ready.get_mut() {
            unsafe {self.message.get_mut().assume_init_drop();}
        }
    }
}
