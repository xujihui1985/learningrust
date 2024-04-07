#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2018::*;
#[macro_use]
extern crate std;
use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::Waker;
use std::thread;
use std::time::Duration;
use pin_project::pin_project;
use futures::FutureExt;
use futures::task::{Context, Poll};
async fn hello_world() {
    {
        ::std::io::_print(::core::fmt::Arguments::new_v1(
            &["hello async world\n"],
            &[],
        ));
    };
}
pub struct TimerFuture {
    shared_state: Arc<Mutex<SharedState>>,
}
impl TimerFuture {
    pub fn new(duration: Duration) -> Self {
        let shared_state = Arc::new(Mutex::new(SharedState {
            completed: false,
            waker: None,
        }));
        let thread_shared_state = shared_state.clone();
        thread::spawn(move || {
            thread::sleep(duration);
            let mut shared_state = thread_shared_state.lock().unwrap();
            shared_state.completed = true;
            if let Some(waker) = shared_state.waker.take() {
                waker.wake()
            }
        });
        TimerFuture { shared_state }
    }
}
struct SharedState {
    completed: bool,
    waker: Option<Waker>,
}
impl Future for TimerFuture {
    type Output = ();
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut shared_state = self.shared_state.lock().unwrap();
        if shared_state.completed {
            Poll::Ready(())
        } else {
            shared_state.waker = Some(cx.waker().clone());
            Poll::Pending
        }
    }
}
#[pin(__private())]
struct MySleep {
    #[pin]
    sleep: tokio::time::Sleep,
}
#[allow(box_pointers)]
#[allow(deprecated)]
#[allow(explicit_outlives_requirements)]
#[allow(single_use_lifetimes)]
#[allow(unreachable_pub)]
#[allow(clippy::unknown_clippy_lints)]
#[allow(clippy::pattern_type_mismatch)]
#[allow(clippy::redundant_pub_crate)]
#[allow(clippy::type_repetition_in_bounds)]
#[allow(unused_qualifications)]
#[allow(clippy::semicolon_if_nothing_returned)]
#[allow(clippy::use_self)]
#[allow(clippy::used_underscore_binding)]
const _: () = {
    #[allow(unused_extern_crates)]
    extern crate pin_project as _pin_project;
    #[allow(dead_code)]
    #[allow(clippy::mut_mut)]
    struct __MySleepProjection<'pin>
    where
        MySleep: 'pin,
    {
        sleep: ::pin_project::__private::Pin<&'pin mut (tokio::time::Sleep)>,
    }
    #[allow(dead_code)]
    #[allow(clippy::ref_option_ref)]
    struct __MySleepProjectionRef<'pin>
    where
        MySleep: 'pin,
    {
        sleep: ::pin_project::__private::Pin<&'pin (tokio::time::Sleep)>,
    }
    impl MySleep {
        fn project<'pin>(
            self: _pin_project::__private::Pin<&'pin mut Self>,
        ) -> __MySleepProjection<'pin> {
            unsafe {
                let Self { sleep } = self.get_unchecked_mut();
                __MySleepProjection {
                    sleep: _pin_project::__private::Pin::new_unchecked(sleep),
                }
            }
        }
        #[allow(clippy::missing_const_for_fn)]
        fn project_ref<'pin>(
            self: _pin_project::__private::Pin<&'pin Self>,
        ) -> __MySleepProjectionRef<'pin> {
            unsafe {
                let Self { sleep } = self.get_ref();
                __MySleepProjectionRef {
                    sleep: _pin_project::__private::Pin::new_unchecked(sleep),
                }
            }
        }
    }
    #[forbid(unaligned_references, safe_packed_borrows)]
    fn __assert_not_repr_packed(this: &MySleep) {
        let _ = &this.sleep;
    }
    #[allow(missing_debug_implementations)]
    struct __MySleep<'pin> {
        __pin_project_use_generics: _pin_project::__private::AlwaysUnpin<'pin, ()>,
        __field0: tokio::time::Sleep,
    }
    impl<'pin> _pin_project::__private::Unpin for MySleep where
        __MySleep<'pin>: _pin_project::__private::Unpin
    {
    }
    #[doc(hidden)]
    unsafe impl<'pin> _pin_project::UnsafeUnpin for MySleep where
        __MySleep<'pin>: _pin_project::__private::Unpin
    {
    }
    trait MySleepMustNotImplDrop {}
    #[allow(clippy::drop_bounds, drop_bounds)]
    impl<T: _pin_project::__private::Drop> MySleepMustNotImplDrop for T {}
    impl MySleepMustNotImplDrop for MySleep {}
    #[doc(hidden)]
    impl _pin_project::__private::PinnedDrop for MySleep {
        unsafe fn drop(self: _pin_project::__private::Pin<&mut Self>) {}
    }
};
impl Future for MySleep {
    type Output = ();
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut this = self.project();
        this.sleep.as_mut().poll(cx)
    }
}
fn main() {
    let mut s = String::from("hello");
    let pin_s = std::pin::Pin::new(&mut s);
    pin_s.get_mut();
}
