use std::rc::Rc;
mod linklist;
mod double_linkedlist;
mod skip_list;
mod dynamic_array;

fn do_something(i: u32) {
}

#[derive(Debug)]
struct MyStruct {
    x: Vec<u32>,
}

#[derive(Debug)]
struct MyStruct2<'a> {
    x: &'a Vec<u32>,
}


#[derive(Debug)]
struct FileNameRef {
    name: Rc<String>,
    ext: Rc<String>,
}
#[derive(Debug)]
struct FileName {
    name: String,
    ext: String,
}

fn print_filename() {
    let name = Rc::new(String::from("hello"));
    let ext = Rc::new(String::from("world"));

    for _ in 0..3 {
        println!("{:?}", FileNameRef{
            name: name.clone(),
            ext: ext.clone(),
        });
    }
}

fn another_function(mut passing_thought: MyStruct) -> MyStruct {
    let v = vec![1,2,3];
    passing_thought.x = v;
    passing_thought
}

fn another_function2<'a>(mut passing_thought: MyStruct2<'a>, x: &'a Vec<u32>) -> MyStruct2<'a> {
    passing_thought.x = x;
    passing_thought
}

fn main() {
    let a = 10;
    let mut ms = MyStruct {
        x: vec![]
    };
    ms = another_function(ms);
    println!("{:?}", ms);
    do_something(a);
    let b = a;
    println!("{}", b);

    print_filename();
    let v = vec![1,2,3];
    let mut ms2 = MyStruct2 {
        x: &v
    };
    ms2 = another_function2(ms2, &v);
    println!("{:?}", ms2);

    use_linklist();
    use_double_linklist();
}

fn use_linklist() {
    let mut log = linklist::TransactionLog::new_empty();
    log.append("world");
    log.append("hello");

    match log.pop() {
        Some(l) => println!("log is {}", l),
        None => println!("empty"),
    };
}

fn use_double_linklist() {
    let mut log = double_linkedlist::BetterTransactionLog::new();
    log.append("hello");
    log.append("world");
    log.append("rust");

    for l in log.iter() {
        println!("{}", l);
    }
}
