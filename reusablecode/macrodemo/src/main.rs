use hello_macro;
use hello_macro::HelloMacro;




// Declarative macro
// vec![] println!()

// Procedual macro
// #[derive(Debug,Clone,Copy)]

macro_rules! hello {
    () => {
        println!("hello");
    };
}

macro_rules! vec {
    ($($x:expr),*) => {
       {
           let mut temp_vec = Vec::new();
           $(
               temp_vec.push($x);
           )*
           temp_vec
       } 
    };
}

fn main() {
    hello!();
    vec!(1,2,3);
}

