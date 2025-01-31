extern crate hello_world_derive;
use hello_world_derive::HelloWorld;

trait HelloWorld {
    fn hello_world(&self);
}

#[derive(HelloWorld)]
struct World;

fn main() {
    let w = World{};
    w.hello_world();
    w.hello_world2();
}
