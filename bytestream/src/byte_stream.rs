use bytes::{Bytes, BytesMut, BufMut};
use futures::{future, stream, Stream, StreamExt};
use pin_project::pin_project;
use std::io;
use std::pin::Pin;
use std::task::{Context, Poll};
use tokio::io::AsyncRead;
use std::io::Read;

#[pin_project]
pub struct ByteStream {
    size_hint: Option<usize>,

    #[pin]
    inner: Pin<Box<dyn Stream<Item = Result<Bytes, io::Error>> + Send + Sync + 'static>>,
}

impl ByteStream {
    pub fn new<S>(stream: S) -> Self
    where
        S: Stream<Item = Result<Bytes, io::Error>> + Send + Sync + 'static,
    {
        Self {
            size_hint: None,
            inner: Box::pin(stream),
        }
    }

    pub fn into_blocking_read(self) -> impl io::Read + Send {
        BlockingReadImpl::new(self.inner)
    }
}

#[pin_project]
struct AsyncReadImpl {
    buffer: BytesMut,
    #[pin]
    stream: futures::stream::Fuse<Pin<Box<dyn Stream<Item = Result<Bytes, io::Error>> + Send>>>,
}

impl AsyncReadImpl {
    fn new(stream: Pin<Box<dyn Stream<Item = Result<Bytes, io::Error>> + Send>>) -> Self {
        AsyncReadImpl {
            buffer: BytesMut::new(),
            stream: stream.fuse(), // stream.fuse turn a stream in to a fused stream, a fused stream will always emit None once stream end
        }
    }
}

impl AsyncRead for AsyncReadImpl {
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &mut [u8],
    ) -> std::task::Poll<std::result::Result<usize, std::io::Error>> {
        let this = self.project();
        if this.buffer.is_empty() {
            match futures::ready!(this.stream.poll_next(cx)) {
                None => return Poll::Ready(Ok(0)),
                Some(Err(e)) => return Poll::Ready(Err(e)),
                Some(Ok(bytes)) => {
                    this.buffer.put(bytes);
                }
            }
        }
        let available = std::cmp::min(buf.len(), this.buffer.len());
        let bytes = this.buffer.split_to(available);
        let (left, _) = buf.split_at_mut(available);
        left.copy_from_slice(&bytes[..available]);
        Poll::Ready(Ok(available))
    }
}

#[pin_project]
struct BlockingReadImpl {
    #[pin]
    inner: AsyncReadImpl,
}

impl BlockingReadImpl {
    fn new(stream: Pin<Box<dyn Stream<Item = Result<Bytes, io::Error>> + Send + Sync>>) -> Self {
        BlockingReadImpl {
            inner: AsyncReadImpl::new(stream),
        }
    }
}

impl io::Read for BlockingReadImpl {
    fn read(&mut self, buf: &mut [u8]) -> std::result::Result<usize, std::io::Error> {
        let mut rt = tokio::runtime::Runtime::new()?;
        rt.block_on(future::poll_fn(|cx| {
            tokio::io::AsyncRead::poll_read(Pin::new(&mut self.inner), cx, buf)
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let s = stream::once(async move { Ok(Bytes::from(vec![1, 2, 3, 4])) });
        let bs = ByteStream::new(s);
        let mut blocking_read = bs.into_blocking_read();
        let mut buf = [0u8; 2];
        assert_eq!(blocking_read.read(&mut buf).unwrap(), 2);
        let expect:[u8; 2] = [1,2];
        assert_eq!(&buf[..2], expect);

        let mut buf = [0u8; 1];
        assert_eq!(blocking_read.read(&mut buf).unwrap(), 1);
        let expect:[u8; 1] = [3];
        assert_eq!(&buf[..1], expect);
    }
}
