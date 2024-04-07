trait MyTrait<T> {
    fn do_something(&self, value: T);
}

//fn foo<'a>(b: Box<dyn MyTrait<&'a usize>>) {
    //let x = 10_usize;
    //b.do_something(&x);
//}

fn bar(b: Box<dyn for<'a> MyTrait<&'a usize>>) {
    let x = 10_usize;
    b.do_something(&x);
}

#[derive(Default)]
struct Config {
    name: String,
}

fn main() {
    let cfg = {
        let mut c = Config::default();
        c.name = "hello".to_string();
        c
    };
}
