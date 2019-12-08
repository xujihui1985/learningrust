
fn sum(v: &Vec<i32>) -> i32 {
    let mut sum = 0;
    for i in v {
        sum += i;
    }
    sum
}

fn cap_values_owned(max: i32, v: &mut Vec<i32>) {
    for i in 0..v.len() {
        if v[i] > max {
            v[i] = max;
        }
    }
    // v
}

fn mut_parm(mut x: i32) {
    x = x + 10;
    println!("x is {}", x);
}

fn main() {

    let mut v = vec![1,2,3,4,5];
    cap_values_owned(4, &mut v);
    println!("result is {:?}", v);
    let x = 1;
    mut_parm(x);
    // let res = sum(&v);
    // println!("sum of v {:?} is {}", v, res);
    //
    let mut values = vec![1,2,3];
    let a = &values;
    println!("{:?}", a);
    values[0] = 4;

    let a = vec![1,2,3,4,5];
    let sa = &a[0..2];

    let b = String::from("hello");
    let sb = &b[0..2];
    println!("sb is {}", sb);

}
