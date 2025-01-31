pub struct ComplexRefs<'a> {
    pub int_ref: &'a i32,
    pub str_ref: &'a str
}

pub fn lifetime_test() {
    
    let a: i32 = 123;
    let b: i32 = 234;
    let c: i32 = 456;

    let d: i32 = 5;
    let str_ref: &str = "hello";
    
    let cr: ComplexRefs = ComplexRefs {
        int_ref: &a,
        str_ref: str_ref,
    };

    let e;
    let f;
    {
        let r = cr;
        e = three_refs(&a, &b, &c, &r);
        f = three_refs2(&a, &b, &c, &r);
    }
    println!("result of e is {}", e);
    println!("result of f is {}", f);
}

fn three_refs<'a, 'b>(a: &'a i32, b: &i32, c: &i32, cr: &'b ComplexRefs<'a>) -> &'a i32 {
    if *a < 5 {
        cr.int_ref
    } else {
        a
    }
}

fn three_refs2(a: &i32, b: &i32, c: &i32, cr: &ComplexRefs) -> i32 {
    if *a < 5 {
        *cr.int_ref
    } else {
        *a
    }
}

// pub fn xxxxx() {
//     let mut s1 = String::from("hello");
//     let s1ref = &s1;
//     s1.push_str("xxx");
//     println!("{}", s1);
//     println!("{}", s1ref);
//     println!("{}", s1);

// }

fn xxxxx2() {
    let mut s1 = String::from("hello");
    let s1ref = &s1;
    println!("{}", s1);
    println!("{}", s1ref);
    s1.push_str("xxx");
    println!("{}", s1);
}
