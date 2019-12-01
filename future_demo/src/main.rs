use std::cell::RefCell;
use std::task::Poll;

thread_local!(static NOTIFY: RefCell<bool> = RefCell::new(true));

struct Waker;

impl Waker {
    fn wake(&self) {
        println!("wake");
        NOTIFY.with(|f| *f.borrow_mut() = true)
    }

    fn hello(&self) {
       println!("hello");
    }
}

fn run<F>(mut f: F) -> F::Output
    where F: Future,
{
   NOTIFY.with(|n| loop {
       println!("before {}", *n.borrow());
       if *n.borrow() {
           *n.borrow_mut() = false;
           println!("after {}", *n.borrow());
           let ctx = Context::from_waker(&Waker{});
           if let Poll::Ready(val) = f.poll(&ctx) {
               return val;
           }
           println!("after poll {}", *n.borrow());
       }
   })
}

struct Context<'a> {
    waker: &'a Waker,
}

trait Future {
    type Output;
    fn poll(&mut self, ctx: &Context) -> Poll<Self::Output>;
}

#[derive(Default)]
struct MyFuture {
    count: i32,
}

struct AddOneFuture<T>(T);

impl<T> Future for AddOneFuture<T>
where T: Future,
      T::Output: std::ops::Add<i32, Output=i32> {
    type Output = i32;
    fn poll(&mut self, ctx: &Context) -> Poll<Self::Output> {
        match self.0.poll(ctx) {
            Poll::Ready(count) => Poll::Ready(count + 1),
            Poll::Pending => Poll::Pending,
        }
    }
}

impl Future for MyFuture {
    type Output = i32;

    fn poll(&mut self, ctx: &Context) -> Poll<Self::Output> {
        match self.count {
            3 => Poll::Ready(3),
            _ => {
                self.count += 1;
                ctx.waker().wake();
                Poll::Pending
            }
        }
    }
}

impl<'a> Context<'a> {
    fn from_waker(waker: &'a Waker) -> Self {
        Context { waker }
    }

    fn waker(&self) -> &'a Waker {
        &self.waker
    }
}

fn use_waker(w: &Waker) {
    w.hello();
}

fn main() {
//    use_waker(&Waker);
    let my_future = MyFuture::default();
    let result = run(my_future);
    println!("output {}", result);
}
