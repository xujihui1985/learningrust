use std::cell::UnsafeCell;
use std::{sync::atomic::AtomicUsize, ptr::NonNull, ops::Deref};
use std::sync::atomic::{Ordering, fence};

struct ArcData<T> {
    // Number of Arc
    data_ref_count: AtomicUsize,

    // number of arc and weak combined
    alloc_ref_count: AtomicUsize,

    // None if there is only weak ref left
    data: UnsafeCell<Option<T>>,
}

unsafe impl<T> Send for Weak<T> where T:Sync + Send {}
unsafe impl<T> Sync for Weak<T> where T:Sync + Send {}

pub struct Weak<T> {
    ptr: NonNull<ArcData<T>>,
}

impl<T> Weak<T> {
    fn data(&self) -> &ArcData<T> {
        unsafe {self.ptr.as_ref()}
    }
}

pub struct Arc<T> {
    weak: Weak<T>,
}

impl<T> Arc<T> {
    pub fn new(data: T) -> Arc<T> {
        Arc {
            // use Box::new to create a new allocation, Box::leak to give up
            // our exclusive ownership of this allocation, and NonNull::from
            // to turn it into a pointer
            weak: Weak {
                ptr: NonNull::from(Box::leak(Box::new(ArcData{
                    alloc_ref_count:AtomicUsize::new(1),
                    data_ref_count: AtomicUsize::new(1),
                    data: UnsafeCell::new(Some(data)),
                })))
            }
        }
    }

    // this function does not take a self argument, but takes a regular argument
    // instread. this means it can only be called as Arc::get_mut(&mut a), and not as a.get_mut()
    // this is advisable for types that implement Deref, to avoid ambiguity with 
    // a similarly named method on the uderling T
    pub fn get_mut(arc: &mut Self) -> Option<&mut T> {
        if arc.data().ref_count.load(Ordering::Relaxed) == 1 {
            fence(Ordering::Acquire);
            unsafe {Some(&mut arc.ptr.as_mut().data)}
        } else {
            None
        }
    }

    fn data(&self) -> &ArcData<T> {
        unsafe {self.ptr.as_ref()}
    }
}

impl<T> Deref for Arc<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        let ptr = self.weak.data().data.get();
        unsafe { (*ptr).as_ref().unwrap() }
    }
}

impl<T> Drop for Arc<T> {
    fn drop(&mut self) {
        if self.weak.data().data_ref_count.fetch_sub(1, Ordering::Release) == 1 {
            fence(Ordering::Acquire);
            let ptr = self.weak.data().data.get();
            unsafe {
                (*ptr) = None;
            }
        }
    }
}

impl<T> Drop for Weak<T> {
    fn drop(&mut self) {
        if self.data().alloc_ref_count.fetch_sub(1, Ordering::Release) == 1 {
            fence(Ordering::Acquire);
            unsafe {
                drop(Box::from_raw(self.ptr.as_ptr()));
            }
        }
    }
}

impl<T> Clone for Weak<T> {
    fn clone(&self) -> Self {
        self.data().alloc_ref_count.fetch_add(1, Ordering::Relaxed);
        Weak {
            ptr: self.ptr,
        }
    }
}

impl<T> Clone for Arc<T> {
    fn clone(&self) -> Self {
        let weak = self.weak.clone();
        self.data().data_ref_count.fetch_add(1, Ordering::Relaxed);
        Arc {
            weak
        }
    }
}
