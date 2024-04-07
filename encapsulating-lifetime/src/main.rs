
impl<'a> Foo<'a> {
    fn len(&self) -> usize {
        self.buff.len()
    }

    fn push(&mut self, c: char) {
        self.buff.push(c)
    }
}

trait WithFoo {
    fn with_foo(&mut self, f: &mut dyn FnMut(&mut Foo));
}


struct Foo<'a> {
    buff: &'a mut String,
}

struct Context<'f, 'a:'f> {
    foo: &'f Foo<'a>,
}

fn main() {
    println!("Hello, world!");
}
