use std::{sync::atomic::{AtomicUsize, Ordering, fence}, ops::Deref, ptr::NonNull};
mod arc_weakref;

struct ArcData<T> {
    ref_count: AtomicUsize,
    data: T,
}

struct Arc<T> {
    ptr: NonNull<ArcData<T>>
}

unsafe impl<T: Send + Sync> Send for Arc<T> {}
unsafe impl<T: Send + Sync> Sync for Arc<T> {}

impl<T> Arc<T> {
    pub fn new(data: T) -> Arc<T> {
        Arc {
            // Can not be box here as box represent exclusive ownership
            ptr: NonNull::from(Box::leak(Box::new(ArcData{
                ref_count: AtomicUsize::new(1),
                data
            }))),
        }
    }

    fn data(&self) -> &ArcData<T> {
        unsafe {self.ptr.as_ref()}
    }

    fn mut_data(&mut self) -> &mut ArcData<T> {
        unsafe {self.ptr.as_mut()}
    }

    // this function takes arc as parameter with purpose, this means
    // it can only be called as Arc::get_mut(&mut a), instead of a.get_mut()
    // this is advisable for type that implement Deref to avoide ambiguity with
    // a similarly named function on the underlying T
    pub fn get_mut(arc: &mut Self) -> Option<&mut T> {
        if arc.data().ref_count.load(Ordering::Relaxed) == 1 {
            fence(Ordering::Acquire);
            Some(&mut arc.mut_data().data)
        } else {
            None
        }
    }
}

impl<T> Deref for Arc<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.data().data
    }
}

impl<T> Clone for Arc<T> {
    fn clone(&self) -> Self {
        self.data().ref_count.fetch_add(1, Ordering::Relaxed);
        Arc {
            ptr: self.ptr
        }
    }
}

impl<T> Drop for Arc<T> {
    fn drop(&mut self) {
        if self.data().ref_count.fetch_sub(1, Ordering::Relaxed) == 1 {
            fence(Ordering::Acquire);
            unsafe {
                drop(Box::from_raw(self.ptr.as_ptr()));
            }
        }
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod test {
    use std::sync::atomic::AtomicUsize;

    use crate::Arc;

    #[test]
    fn test() {
        static NUM_DROP: AtomicUsize = AtomicUsize::new(0);
        struct DetectDrop;
        impl Drop for DetectDrop {
            fn drop(&mut self) {
                NUM_DROP.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            }
        }

        let x = Arc::new(("hello", DetectDrop));
        let mut y = x.clone();

        let t = std::thread::spawn(move || {
            assert_eq!(x.0, "hello");
        });
        assert_eq!(y.0, "hello");

        t.join().unwrap();

        assert_eq!(NUM_DROP.load(std::sync::atomic::Ordering::Relaxed), 0);

        let a = Arc::get_mut(&mut y);
        assert!(a.is_some());

        drop(y);
        assert_eq!(NUM_DROP.load(std::sync::atomic::Ordering::Relaxed), 1);

    }
}
