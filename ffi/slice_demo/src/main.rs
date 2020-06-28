use std::{borrow::Cow, marker};
use std::slice;
use std::mem::{transmute, size_of};

fn main() {
    println!("Hello, world!");
}

#[repr(C)]
struct Slice<T> {
    offset: u32,
    len: u32,
    _phantom: marker::PhantomData<T>,
}

struct Header {
    targets: Slice<u32>,
}

struct Data<'a> {
    bytes: Cow<'a, [u8]>,
}

impl <'a> Data<'a> {
    pub fn new<B: Into<Cow<'a, [u8]>>>(bytes: B) -> Self {
        Data {bytes: bytes.into()}
    }

    pub fn get_target(&self, idx: usize) -> u32 {
        self.load_slice(&self.header().targets)[idx]
    }

    fn bytes(&self, start: usize, len: usize) -> *const u8 {
        self.bytes[start..start + len].as_ptr()
    }

    fn header(&self) -> &Header {
        unsafe {
            transmute(self.bytes(0, size_of::<Header>()))
        }
    }

    fn load_slice<T>(&self, s:&Slice<T>) -> &[T] {
        let size = size_of::<T>() * s.len as usize;
        let bytes = self.bytes(s.offset as usize, size);
        unsafe {slice::from_raw_parts(bytes as *const T, s.len as usize)}
    }
}