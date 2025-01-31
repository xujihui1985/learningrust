use std::i8;

fn main() {
    // let s = String::from("hello");
    let a = i8::MIN;
    let b = i8::MAX;

    println!("{} {}", a, b); // -128 -> 127
    // Encode(x) == 0
    // let value: i8 = 0b10000000u8 as i8;
    let encoded = a ^ 0b10000000u8 as i8;
    println!("value: {}", encoded);
    let decode = encoded ^ 0b10000000u8 as i8;
    println!("decode: {}", decode);

    

//    take_ownership(s);
 //   println!("{}", s); // s has been moved into fn `take_ownership`
}

fn take_ownership(s: String) { // s comes into scope
    println!("{}", s);
} // s goes out of scope and `drop` is called. backing memory is freed.
