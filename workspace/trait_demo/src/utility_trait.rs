use std::fmt::Display;
use std::fs::File;
use std::ops::{Deref, DerefMut};
use std::path::Path;
use std::net::Ipv4Addr;

// ?Sized means unsized
struct RcBox<T: ?Sized> {
    ref_count: usize,
    value: T,
}

fn display(boxed: &RcBox<Display>) {
    println!("{}", &boxed.value);
}

struct Selector<T> {
    elements: Vec<T>,
    current: usize,
}

impl<T> Deref for Selector<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.elements[self.current]
    }
}

impl<T> DerefMut for Selector<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.elements[self.current]
    }
}

fn open<P>(path: P) -> std::io::Result<File>
    where P: AsRef<Path>
{
    let p = path.as_ref();
}

fn ping<A>(address: A) -> std::io::Result<bool>
    where A:Into<Ipv4Addr>
{
    //  use Into to make your functions more flexible in the arguments they accept. For example, if you write:
    let ipv4_address = address.into();

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_work() {
        let boxed_lunch: RcBox<String> = RcBox {
            ref_count: 1,
            value: "lunch".to_string(),
        };
        display(&boxed_lunch);
    }

    #[test]
    fn test_deref_selector() {
        let s = Selector {
            elements: vec!['x', 'y', 'z'],
            current: 1,
        };

//        assert_eq!(*s, 'y');
        // assert that 'z' is alphabetic, using a method of `char` directly on
        // `Selector`, via deref coercion
        assert!(s.is_alphabetic());
    }

    #[test]
    fn test_asref() {}
}