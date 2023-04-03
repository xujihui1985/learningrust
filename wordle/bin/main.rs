use std::marker::PhantomData;
use std::sync::Arc;

fn main() {
    // let a = vec![123];
    // let f = Shared {
    //     inner: Arc::new(a)
    // };

    let b = Bar{name: String::from("hello")};
    let a = &Bar{name: String::from("world")};

    call_bar(b);
    call_bar(a);
}

fn call_bar(t: impl Fooer) {
    t.hello();
}

trait Fooer {
  fn hello(&self);
}

struct Bar{
    name: String,
}

impl Fooer for Bar {
    fn hello(&self) {
        todo!()
    }
}

impl Fooer for &Bar {
    fn hello(&self) {
        let a = *self;
        println!("hello from &bar {}", a.name)
    }
}
mod sealed {
    pub trait Sealed{}
}

trait MyTrait<T, V>: sealed::Sealed{
    fn hello(a: T, b: V);
}

// #[derive(Clone)]
// struct Shared<T> {
//    inner: Arc<T>
// }

struct Grounded;
struct Launched;

struct Rocket<Stage = Grounded> {
    stage: std::marker::PhantomData<Stage>
}

impl Default for Rocket<Grounded> {
    fn default() -> Self {
       Rocket{stage: PhantomData}
    }
}

impl Rocket<Grounded> {
    pub fn launch(self) -> Rocket<Launched> {
        Rocket{stage: PhantomData}
    }
}
impl Rocket<Launched> {
    pub fn accelerate(&mut self) { }
    pub fn decelerate(&mut self) { }
}

impl<Stage> Rocket<Stage> {
    pub fn color(&self) {}
    pub fn weight(&self) {}
}
