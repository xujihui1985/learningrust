fn main() {
    let s = String::from("hello");
    take_ownership(s);
    println!("{}", s); // s has been moved into fn `take_ownership`
}

fn take_ownership(s: String) { // s comes into scope
    println!("{}", s);
} // s goes out of scope and `drop` is called. backing memory is freed.
