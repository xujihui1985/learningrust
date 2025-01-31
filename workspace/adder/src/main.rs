use std::path::Path;

extern crate add_one;
mod submodule;

fn main() {
    let res = add_one::add_one(123);
    {
        let sp = add_one::CustomSmartPointer{
            data: String::from("hello"),
        };
        drop(sp); // use drop to drop memory early
        println!("Hello, world2! ");
    }
    println!("Hello, world! {}", res);
    let h = submodule::roots::Hello{
        name: "sean".to_string(),
    };
    println!("{:?}", h);
}

fn handle_option() {
    let p = Path::new("src/main.rs");
    let p2 = Path::new(&&&p);
    let some_u8_value = Some(0u8);
    let Some(v) = some_u8_value else {
        println!("three");
        return;
    };
    println!("{}", v)
}