use std::mem;
use std::collections::HashMap;
use std::fs::File;
mod stackheap;
mod control_flow;

const GLOBAL_CONST:u8 = 42;

static STATIC_GLOBAL:i32 = 321;
static mut STATIC_MUT_GLOBAL:i32 = 4321;

fn main() {
    let a:u8 = 123; // 8bit unsigned integer
    println!("a = {}",a);

    let mut b = 11; // 8bit signed integer
    println!("b = {}",b );
    b = 22;
    println!("b = {}",b );

    let c = 123456789;
    println!("c = {}, size = {} bytes", c, mem::size_of_val(&c));  // 4 bytes 32 bit

    let z:isize = 123; // isize is the size of pointer, like if you had an address in memory, what would 
    // the size of that
    let size_of_z = mem::size_of_val(&z);
    println!("z takes up {} bytes, {}-bit os", size_of_z, size_of_z * 8);

    println!("global const {}", GLOBAL_CONST);
    println!("static global {}", STATIC_GLOBAL);
    use_global_static_mut();

    stackheap::stack_and_heap();
    control_flow::if_statement();
    get_char();
    zip_hashmap();
    count_words();
    open_file();
}

fn use_global_static_mut() {
    unsafe {
        println!("static mut global {}", STATIC_MUT_GLOBAL);
    }
}

// fn vector() {
//     let mut v = vec![1,2,3];
//     let first = &v[0];
//     v.push(4); // cannot borrow `v` as mutable because it is also borrowed as immutable
// }

fn update_string() {
    let mut s = String::from("hello");
    s.push_str("world");
}

fn format_str() {
    let s1 = String::from("hello");
    let s2 = String::from("world");

    let s = format!("{}-{}", s1, s2);
}

fn get_char() {
    for c in "中文".chars() {
        println!("{}",c);
    }
}

fn use_hashmap() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yello"), 20);
}

fn zip_hashmap() {
    let teams = vec!["Blue", "yello"];
    let initial_scores = vec![10,20];
    let scores: HashMap<_,_> = teams.iter().zip(initial_scores.iter()).collect();
    print!("{:?}", scores);
}

fn insert_value_if_key_no_value() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Blue")).or_insert(5);
}

fn count_words() {
    let text = "hello world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);
}

fn open_file() {
    let f = File::open("./hello");
    let f = match f {
        Ok(file) => file,
        Err(e) => {
            panic!("failed to open file {:?}", e)
        },
    };
}

fn error_handle_shortcut() {
    let f = File::open("./world").unwrap();
    let f1 = File::open("./world").expect("custom error");
}
