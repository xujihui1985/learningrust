#[derive(Debug)]
struct Hello {
    int_field: i32,
    string_field: String,
}

fn move_struct() {
    let mut a = Hello {
        int_field: 1,
        string_field: String::from("hello"),
    };

    let ptr_a = &mut a as *mut Hello;
    println!("ptr_a {:?}", ptr_a);
    let mut b = a;
    let ptr_b = &mut b as *mut Hello;
    println!("ptr_b {:?}", ptr_b);
    b.int_field = 2;
    b.string_field = String::from("world");

    println!("ref_b {:?}", &b);
    unsafe{
        let a = &*ptr_a;
        println!("ref_a {:?}", &a);
    }

}

fn move_vec() {

    let mut a = vec![1,3,4];

    let ptr_a = &mut a as *mut Vec<i32>;
    println!("ptr_a {:?}", ptr_a);

    let mut b = a;
    let ptr_b = &mut b as *mut Vec<i32>;
    println!("ptr_b {:?}", ptr_b);

    b.push(5);
    b.push(6);
    b.push(7);
    b.push(8);
    
    println!("ref_b {:?}", &b);

    unsafe{
        let a = &*ptr_a;
        println!("ref_a {:?}", &a);
    }
}


fn main() {
    move_struct();
    // move_vec();
}