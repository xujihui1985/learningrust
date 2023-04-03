fn serve_order() {
    println!("serve order");
}

mod back_of_house {
    pub fn fix_incorrect_order() {
        crate::serve_order();
        super::serve_order();
    }
}

struct TestStruct {
}

impl TestStruct {
    fn return_string(&self) -> String {
        String::from("hello")
    }
}

fn main() {
    let name = env::args().skip(1).next();
    match name {
        Some(n) => println!("Hi there {}", n),
        None => println!("didn't receive any name")
    }
    back_of_house::fix_incorrect_order();
    temporary_value_is_freed();
}

// 
pub fn temporary_value_is_freed() {
    let t = TestStruct{};
    //let hello = t.return_string().as_str();
    let hello = t.return_string();
    let hello = hello.as_str();
    //let hello = {
        //let __temp = t.return_string();
        //__temp.as_str()
    //};
    println!("{}", hello);
}


