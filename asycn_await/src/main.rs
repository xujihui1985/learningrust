// mod pin;
use pin_project::pin_project;
use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::Waker;
use std::thread;
use std::time::Duration;

use futures::task::{Context, Poll};
use futures::FutureExt;
// use tokio::macros::support::Pin;

// it is same as future::Future

async fn hello_world() {
    println!("hello async world");
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

#[pin_project]
struct MySleep {
    #[pin]
    sleep: tokio::time::Sleep,
}

struct World;

struct Hello<'pin>
where
    World: 'pin,
{
    hello: &'pin str,
}

impl Future for MySleep {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut this = self.project();
        this.sleep.as_mut().poll(cx)
    }
}

fn main() {
    // let mut rt = tokio::runtime::Builder::new().enable_all().build().unwrap();
    // rt.block_on(TimerFuture::new(Duration::from_millis(2000)));

    let mut s = String::from("hello");
    // tokio::pin!(s);
    let pin_s = std::pin::Pin::new(&mut s);
    pin_s.get_mut();

    let h = Hello{
        hello: "aaa"
    };
#[tokio::main]
async fn main() {
    println!("should sleeping for 1 second");
    let t = TimerFuture::new(Duration::from_millis(1000));
    t.await;
    hello_world().await;
    let v = 123;
    let l = tokio::sync::RwLock::new(v);

    let g = l.read().await;
    println!("got read lock");
    drop(g);

    let wg = l.write().await;
    println!("got write lock");
}
