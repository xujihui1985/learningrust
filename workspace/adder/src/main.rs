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
