use std::fmt::Debug;
mod reverse_contrain;

#[derive(Debug)]
struct Square {
    x: u32,
    y: u32,
}

// where clause on struct
struct Point<P> where P: Debug {
    x: P,
    y: P,
}

fn print<S>(s: &S)
    where S: Debug {
        println!("{:?}", s);
}

fn main() {
    let s = Square{
        x: 123,
        y: 456,
    };
    print(&s);
}

// multiple constrain
fn some_function<T, E>(a: T, b: E)
    where T: Debug,
          T: Send + Sync + 'static,
          E: Debug + Send + Sync
{
}
