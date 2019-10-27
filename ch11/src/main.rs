use std::io::Write;
use std::fmt::Debug;
use std::hash::Hash;
use std::ops::{Add, Mul};

fn main() {
    let mut buf: Vec<u8> = vec![];
    buf.write_all(b"hello").unwrap();

    let b = Broom {x: 1, y: 2};
    b.hit_test(3, 4);
}

fn top_ten<T> (values: &Vec<T>)
    where T:Debug+Hash+Eq 
{
} 
trait Visible {

    fn hit_test(&self, x:i32, y:i32) -> bool;
}

struct Broom {
    x: i32,
    y: i32,
}

impl Visible for Broom {
    
    fn hit_test(&self, x:i32, y:i32) -> bool {
        true
    }
}

trait Clone {
    fn clone(&self) -> Self;
}

trait StringSet {
    fn new() -> Self
        where Self: Sized; //static method

    fn add(&mut self, string: &str);
    fn contains(&self, string: &str) -> bool;
}

fn unknown_words<S: StringSet>(document: &Vec<String>, wordlist: &S) -> S {
    let mut unknown = S::new();
    for word in document {
        if !wordlist.contains(word) {
            unknown.add(word);
        }
    }
    unknown
}

impl Clone for Broom {
    fn clone(&self) -> Self {
        Broom {x: 1, y: 2}
    }
}

fn non_generic(v1: &[i64], v2: &[i64]) -> i64 {
    let mut total = 0;
    for i in 0..v1.len() {
        total = total + v1[i] * v2[i];
    }
    total
}

//fn generic_v1<N>(v1: &[N], v2: &[N]) -> N {
    //let mut total: N = 0;
    //for i in 0..v1.len() {
        //total = total + v1[i] * v2[i];
    //}
    //total
//}

fn generic_v2<N>(v1: &[N], v2: &[N]) -> N 
    where N:Add<Output=N> + Mul<Output=N> + Default + Copy
{
    let mut total = N::default();
    for i in 0..v1.len() {
        total = total + v1[i] * v2[i];
    }
    total
}

#[test]
fn test_generic() {
    assert_eq!(generic_v2(&[1,2,3], &[1,2,3]), 14);
}
