fn main() {
    let v = vec![String::from("hello"), String::from("world"),];

    for s in v {
        consume_string(s);
    }

}

fn consume_string(s: String) {
    println!("{}", s);
}

