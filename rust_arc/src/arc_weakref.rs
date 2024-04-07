use std::{sync::atomic::AtomicUsize, cell::UnsafeCell, ptr::NonNull, ops::Deref};

struct ArcData<T> {
    data_ref_count: AtomicUsize,
    alloc_ref_count: AtomicUsize,
    // None if weak pointer left
    data: UnsafeCell<Option<T>>,
}

pub struct Arc<T> {
    weak: Weak<T>
}

impl<T> Deref for Arc<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        let ptr = self.weak.data().data.get();
        unsafe {(*ptr).as_ref().unwrap()}
    }
}


struct Weak<T> {
    ptr: NonNull<ArcData<T>>
}

impl<T> Weak<T> {
    fn data(&self) -> &ArcData<T> {
        unsafe {self.ptr.as_ref()}
    }
}

unsafe impl<T: Sync + Send> Send for Weak<T> { }
unsafe impl<T: Sync + Send> Sync for Weak<T> { }

impl<T> Arc<T> {
    pub fn new(data: T) -> Arc<T> {
        Arc {
            weak: Weak {
                ptr: NonNull::from(Box::leak(Box::new(ArcData{
                    alloc_ref_count: AtomicUsize::new(1),
                    data_ref_count: AtomicUsize::new(1),
                    data: UnsafeCell::new(Some(data)),
                })))
            }
        }
    }
}

