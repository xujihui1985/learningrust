use std::fmt::Debug;
use std::fs::File;
use std::io::Write;
use std::ops::{Add, Not};
mod utility_trait;

fn say_hello(out: &mut Write) -> std::io::Result<()> {
    out.write_all(b"hello world")?;
    out.flush()
}

fn top_ten<T: std::fmt::Debug + Eq>(values: &Vec<T>) {}

// use where keyword
fn top_ten2<T>(values: &Vec<T>)
    where T: std::fmt::Debug + Eq
{}

trait Visible {
    fn hit_test(&self, x: i32, y: i32) -> bool;
}

// subtraits, everytype that implements creature
// must also implement the visible trait
trait Creature: Visible {
    fn position(&self) -> (i32, i32);
}

fn dump<T>(iter: T)
    where T: Iterator, T::Item: Debug {
    for (index, value) in iter.enumerate() {
        println!("{}, {:?}", index, value);
    }
}

// T default to Self
/**
The syntax RHS=Self means that RHS defaults to Self.
If I write impl Mul for Complex, without specifying Mul’s type parameter,
it means impl Mul<Complex> for Complex
*/
pub trait Mul<T = Self> {
    type Output;
    fn mul(self, rsh: T) -> Self::Output;
}

struct Broom {
    x: i32,
    y: i32,
    height: i32,
}

impl Broom {
    fn actual_height(&self) -> i32 {
        self.y - self.height - 1
    }
}

impl Visible for Broom {
    fn hit_test(&self, x: i32, y: i32) -> bool {
        self.x == x
            && self.actual_height() <= y
            && y <= self.y
    }
}

fn dot(v1: &[i64], v2: &[i64]) -> i64 {
    let mut total = 0;
    for i in 0..v1.len() {
        total = total + v1[i] * v2[i];
    }
    total
}

fn dot_gen<T>(v1: &[T], v2: &[T]) -> T
    where T: Add<Output=T> + std::ops::Mul<Output=T> + Default + Copy
{
    let mut total = T::default();
    for i in 0..v1.len() {
        total = total + v1[i] * v2[i];
    }
    total
}

impl Add for Broom {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Broom{
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            height: self.height + rhs.height,
        }
    }
}

impl Not for Broom {
    type Output = bool;
    fn not(self) -> Self::Output {
        self.x < 0
    }
}


#[cfg(test)]
mod tests {
    use std::fs::File;

    use super::*;

    #[test]
    fn test_say_hello() {
        let mut bytes: Vec<u8> = vec![];
        let res = say_hello(&mut bytes);
        assert_eq!(res.is_ok(), true);
        assert_eq!(bytes, b"hello world");
    }

    #[test]
    fn test_say_hello_to_file() {
        let mut local_file = File::create("hello.txt").expect("failed to create");
        let res = say_hello(&mut local_file);
        assert_eq!(res.is_ok(), true);
    }

    #[test]
    fn test_dump() {
        dump(1..10);
    }

    #[test]
    fn test_dot() {
        let res = dot(&[1, 2, 3, 4, 5], &[5, 4, 3, 2, 1]);
        assert_eq!(res, 35);
    }

    #[test]
    fn test_dot_gen() {
        let res = dot_gen(&[1, 2, 3, 4, 5], &[5, 4, 3, 2, 1]);
        assert_eq!(res, 35);
    }

    #[test]
    fn add_broom() {
        let b1 = Broom {
            x: 1,
            y: 2,
            height: 3,
        };
        let b2 = Broom {
            x: 1,
            y: 2,
            height: 3,
        };
        let b3 = b1 + b2;
        assert_eq!(b3.x, 2);
        assert_eq!(b3.y, 4);
        assert_eq!(b3.height, 6);
    }

    #[test]
    fn not_broom() {
        let b1 = Broom {
            x: 1,
            y: 2,
            height: 3,
        };
        assert_eq!(!b1, false);
    }
}
