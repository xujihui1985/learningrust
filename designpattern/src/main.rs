#[derive(Debug, Clone, Copy)]
struct Hello {
    age: i32,
}

fn main() {
    let a = Hello { age: 10 };
    let b = a;
    println!("{:?} is alive", a);

    let ele1 = Some("hello");
    let arr = vec!["aaaa", "bbbb"];

    for e in arr.iter().chain(ele1.iter()) {
        println!("{} is element", e)
    }
}
