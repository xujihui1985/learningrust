use std::ops::Add;
use std::cmp::PartialEq;
use hello_utils::utils::*;
use hello_utils::submod;

struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn get_x(&self) -> &T {
        &self.x
    }
}

fn largest<T>(list: &[T]) -> T 
    where T: PartialOrd + Copy {
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
           largest = item; 
        }
    }
    largest
}


trait NoisyAnimal {
    fn make_noise(&self) -> &'static str;
}

struct Cat{}
struct Dog{}

impl NoisyAnimal for Cat {
    fn make_noise(&self) -> &'static str {
        "hello"
    }
}

impl NoisyAnimal for Dog {
    fn make_noise(&self) -> &'static str {
        "bark"
    }
}

impl PartialEq for Dog {
    fn eq(&self, other: &Dog) -> bool {
       true 
    }
}

fn make_noises(animals: Vec<Box<dyn NoisyAnimal>>) {
    for a in animals.iter() {
        println!("{}", a.make_noise())
    }
}

trait Iterator {
    type Item;
    fn next(&self) -> Option<Self::Item>;
}

struct Counter;

impl Iterator for Counter {
    type Item = u32;

    fn next(&self) -> Option<Self::Item> {
        Some(42)
    }
}

impl Add for Dog {
    type Output = Dog;

    fn add(self, other: Dog) -> Dog {
        Dog{}
    }
}


fn main() {
    let p = Point{x: 5, y: 5};
    println!("x is {}", p.get_x());

    let cat = Cat{};
    cat.make_noise();

    let animals: Vec<Box<dyn NoisyAnimal>> = vec![
        Box::new(Cat{}),
        Box::new(Dog{}),
    ];
    make_noises(animals);
    let c = Counter{};
    let n = c.next().unwrap();

    let d1 = Dog{};
    let d2 = Dog{};
    if d1 == d2 {
        println!("d1 is same as d2");
    }

    submod::print::print_hello();
}
