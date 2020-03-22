
fn main() {
    let mut s = String::from("hello");
    // move_s(s);  // because s is moved, use after moved can not be compiled
    mutable_borrow(&mut s); 

    let s2 = &s;
    println!("s2 is {}", s2);
}

fn mutable_borrow(s: &mut String) {
    let b = s; // s is moved to b
    println!("mutable_borrow {}", b);
}

fn borrow(s: &String) {
}

fn move_s(s1: String) {
    println!("s1 is moved {}", s1);
}