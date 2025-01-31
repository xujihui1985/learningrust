use std::pin::Pin;
use futures::FutureExt;

use futures::{AsyncRead, Future};
// parsed ref to buf
// once it is parsed, you can no longer move the buf
// if you move the buf, then parsed will become darling poiner
pub struct OwnedParse<'a> {
    buf: Vec<u8>,
    parsed: Parsed<'a>,
}

pub struct Parsed<'a> {
    token: &'a [u8],
}

impl <'a> OwnedParse<'a> {
    pub fn set_parsed(&'a mut self) {
        self.parsed = Parsed{
            token: self.buf.as_slice(),
        };
    }

}

// because the return value of foo which is a future
// must reference buf
async fn foo(s: &mut dyn AsyncRead + AsyncWrite) {
    let mut buf = [0; 1024];
    tokio::io::read(s, &mut buf[..]).await
    // suppose tokio::io::read() return a future type IoRead
    // and the method foo itself also return a Future, we call it FooFuture here
}

struct IoRead<'a> {
    buf: &'a mut [u8],
    s: &'a mut dyn AsyncRead,
}

impl<'a> Future for IoRead<'a> {
    type Output = std::io::Result<usize>;

    fn poll(self: Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
    }
}

// this future has a reference to buf,
// async fn foo(s: &mut dyn AsyncRead + AsyncWrite) {
//     let mut buf = [0; 1024];
//     tokio::io::read(s, &mut buf[..]).await
// }
// suppose foo return a future FooFuture, the buf field is the buf
// of async method foo
// if FooFuture was moved, the buf was moved, then f field which
// reference buf is invalid, so FooFuture can not be moved
// struct FooFuture {
//     buf: [u8; 1024],
//     f: IoRead<'same as buf>  // this future has reference to buf
// }


// if we think about memory, move is actually memcpy, f was memcpy to z, and buf was also cpy from f to z
// the memory location of buf in z is different from that in f, but IoRead in z is still point to the location
// where original f is, even buf in z is in new memory location
// what Pin doing is once you five pin, you promise that T will never move after word. once start pulling it's
// no longer okey for FooFuture to move
// compiler automatically implement !Unpin for FooFuture, so that you can not get mutatable reference from poll
// safely
fn call_foo() {
    let f:FooFuture = foo(); // first we call foo that will return a FooFuture f
    f.f.poll(); // then we call poll to poll the future
    let z = f; // here is the problem, f was moved, so as the buf field, the memory location was changed
    z.f.poll();
}

struct IoRead<'a> {
    buf: &'a mut [u8],
    s: &mut dyn AsyncRead,
}

 impl<'a> Future for IoRead<'a> {
     type Output = io::Result<usize>;

    fn poll(self: Pin<&mut Self>) -> Poll<Self::Output> {
    }
}


struct MyFuture<F> where F:Unpin{
    f: F
}

impl<F: Future> Future for MyFuture<F> {
    type Output = F::Output;

    fn poll(self: Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
        self.f.poll_unpin(cx)
        self.f.poll(cx)
    }
}
