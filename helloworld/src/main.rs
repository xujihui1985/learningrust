use std::env;

fn serve_order() {
    println!("serve order");
}

mod back_of_house {
    pub fn fix_incorrect_order() {
        crate::serve_order();
        super::serve_order();
    }
}

fn main() {
    let name = env::args().skip(1).next();
    match name {
        Some(n) => println!("Hi there {}", n),
        None => println!("didn't receive any name")
    }

    back_of_house::fix_incorrect_order();
}
