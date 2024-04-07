fn main() {
    println!("Hello, world!");
    let x = bar; // x is a function item, zero sized item
    println!("{}", std::mem::size_of_val(&x)); // size is 0, function item only
                                               // exists in compile time, so the size of it is 0
    baz(x);
    quox(x);

    let f = |x: u32| x;

    let x = || 0; //const clousure
}

fn bar(_: u32) -> u32 {
    0
}

fn baz(f: fn(u32) -> u32) {
    // here f is a function pointer
    println!("{}", std::mem::size_of_val(&f));
}

// Fn (trait bound) is different from fn (function pointer),
fn quox<F>(f: F)
where
    F: Fn(u32) -> u32,
{
    println!("{}", std::mem::size_of_val(&f));
}

// there are three different trait
// Fn FnMut FnOnce

fn hello(f: Box<dyn Fn()>) {
    f()
}

// const fn
// the clousure that doesn't reference any enclosed veriable is const fn
const fn foo() -> i32 {
    0
}

// const fn foo2<F: FnOnce()>(f: F) {
//     f();
// }

fn bar2<F>(f: F)
where
    F: for<'a> Fn(&'a str) -> &'a str,
{
}
