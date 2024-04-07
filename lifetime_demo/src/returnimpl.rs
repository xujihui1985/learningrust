use std::{future::Future, fmt::Display};

fn main() {
    
}


fn only_true<I>(iter: I) -> impl Future<Output = impl Iterator<Item = bool>>
where I: Iterator<Item = bool> {
    async move {
        iter.filter(|&x| x == true)
    }
}

// not same
// fn f1<R:Trait>() -> R{} // the caller chooses the concrete type
// fn f2<() -> impl Trait {} // the impl Trait is chosen by the function

// autotrait are preserved

// lifetime
// impl trait does not capture lifetime parameters

// the impl Sized return type does not capture the t
fn gen<'a>(t: &'a()) -> impl Sized {
    ()
}

fn foo() -> impl Sized + 'static {
    let x = ();
    gen(&x)
}

// it does capture T, so this not compile

fn gen2<T>(t: T) -> impl Sized {
    ()
}

// fn foo2() -> impl Sized + 'static {
//     let s = String::from("hello");
//     gen2(&s)
// }

// lives at least as long as 'a
// In this context, MyTrait does not take a lifetime parameter itself, but the type that implements MyTrait is guaranteed to live at least as long as 'a.
/*
impl MyTrait + 'a: The lifetime of the return type is at least 'a. It won't be shorter than 'a, but it could be longer.
impl MyTrait<'a>: The lifetime of the return type is specified to be 'a. Any references in the type must have the lifetime 'a, no longer, no shorter.
 */
fn gen3<'a>(t: &'a ()) -> impl Sized + 'a {
    t
}

// fn test4<'a>(s: &'a str) -> impl MyTrait + 'a {
//     s.to_string()
// }

// fn test5<'a>(s: &'a str) -> impl MyTrait<'a> {
//     s.to_string()
// }

struct MyStruct;

trait MyTrait {
    fn do_something(&self);
}

impl MyTrait for MyStruct {
    fn do_something(&self) {
        println!("Doing something!");
    }
}

impl MyStruct {
    fn foo(&self) -> impl MyTrait  {
        MyStruct
    }
}

fn gen8<'a>(t: &'a ()) -> impl Sized + 'a {
    t
}

// trait MyTrait<'a> {
    
// }
// impl<'a> MyTrait<'a> for String {}


fn f1<T> (display: T) 
where T: Display
{

}

fn f2(display: impl Display) 
{

}



fn gen10(t: &str) -> impl Sized{
   ()
}

fn caller() -> impl Sized + 'static {
    let x = String::from("hello");
    gen10(&x)
}

trait MyTrait2<'a> {
    type Item;
 }
 
 impl <'a> MyTrait2<'a> for () {
    type Item = MyStruct2<'a>;
 }
 
 struct MyStruct2<'a>(&'a str);
 impl<'a> MyStruct2<'a> {
     fn impl_trait() -> impl MyTrait2<'a, Item = Self> { 
         ()
     }
 }


fn gen9<T>(t: &T) -> impl Sized {
    ()
}

fn foo9() -> impl Sized + 'static {
    let s = String::from("hello");
    // Here we are simply discarding the reference to `s`
    // and returning `()`, which is 'static.
    gen9(&s)
}

//difference
// f1::<u32>(1) is fine
// f2::<???>() can not specify concrete type


// only nightly now
// trait MyTrait2 {
//     type MyItem = impl Display;
// }

// // type alias impl trait
// // Type = impl Trait

// // return impl trait in trait
// trait MyTrait3 {
//     fn foo() -> impl Display;
// }