use std::{sync::atomic::AtomicUsize, ptr::NonNull, ops::Deref};
use std::sync::atomic::{Ordering, fence};

struct ArcData<T> {
    ref_count: AtomicUsize,
    data: T,
}

unsafe impl<T> Send for Arc<T> where T:Sync + Send {}
unsafe impl<T> Sync for Arc<T> where T:Sync + Send {}

pub struct Arc<T> {
    ptr: NonNull<ArcData<T>>,
}

impl<T> Arc<T> {
    pub fn new(data: T) -> Arc<T> {
        Arc {
            // use Box::new to create a new allocation, Box::leak to give up
            // our exclusive ownership of this allocation, and NonNull::from
            // to turn it into a pointer
            ptr: NonNull::from(Box::leak(Box::new(ArcData{
                ref_count: AtomicUsize::new(1),
                data,
            })))
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
        &self.data().data
    }
}

impl<T> Drop for Arc<T> {
    fn drop(&mut self) {
        if self.data().ref_count.fetch_sub(1, Ordering::Release) == 1 {
            fence(Ordering::Acquire);
            unsafe {
                drop(Box::from_raw(self.ptr.as_ptr()));
            }
        }
    }
}

impl<T> Clone for Arc<T> {
    fn clone(&self) -> Self {
        self.data().ref_count.fetch_add(1, Ordering::Relaxed);
        Arc {
            ptr: self.ptr,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_arc() {
        static NUM_DROPS: AtomicUsize = AtomicUsize::new(0);

        struct DetectDrop;

        impl Drop for DetectDrop {
            fn drop(&mut self) {
                NUM_DROPS.fetch_add(1, Ordering::Relaxed);
            }
        }

        let x = Arc::new(("hello", DetectDrop));
        let y = x.clone();
        let t = std::thread::spawn(move || {
            assert_eq!(x.0, "hello");
        });

        assert_eq!(y.0, "hello");

        t.join().unwrap();

        assert_eq!(NUM_DROPS.load(Ordering::Relaxed), 0);

        drop(y);
        assert_eq!(NUM_DROPS.load(Ordering::Relaxed), 1);
    }

}
