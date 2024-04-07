use std::{rc::Rc, sync::Arc, collections::vec_deque::Iter};

trait HasNumbers {
    const NUMBERS: [i32; 5];
    const LAST_NUMBER: i32 = 5;
}

struct IHaveNumbers;

impl HasNumbers for IHaveNumbers {
    const NUMBERS: [i32; 5] = [1,2,3,4, IHaveNumbers::LAST_NUMBER];
}

trait PointerFamily {
    type PointerType<T>;
}

struct RcPointer;

impl PointerFamily for RcPointer {
    type PointerType<T> = Rc<T>;
}
struct ArcPointer;

impl PointerFamily for ArcPointer {
    type PointerType<T> = Arc<T>;
}

struct MyStruct<K:PointerFamily> {
    num: K::PointerType<i32>,
    str: K::PointerType<String>,
}

trait FunctorFamily {
    type Type<T>;

    fn fmap<T, U, F>(value: Self::Type<T>, f: F) -> Self::Type<U>
    where
        F: FnMut(T) -> U;

    fn new<T>(value: T) -> Self::Type<T>;
}
trait ApplicativeFamily: FunctorFamily {
    fn pure<T>(inner: T) -> Self::Type<T>;

    fn apply<T, U, F>(value: Self::Type<T>, f: Self::Type<F>) -> Self::Type<U>
    where
        F: FnMut(T) -> U;
}

trait MonadFamily: ApplicativeFamily {
    fn bind<T, U, F>(value: Self::Type<T>, f: F) -> Self::Type<U>
    where
        F: FnMut(T) -> Self::Type<U>;
}

struct OptionType;

impl FunctorFamily for OptionType {
    type Type<T> = Option<T>;

    fn fmap<T, U, F>(value: Self::Type<T>, f: F) -> Self::Type<U>
    where
        F: FnMut(T) -> U 
    {
        value.map(f)
    }

    fn new<T>(value: T) -> Self::Type<T> {
        Some(value)
    }
}

impl ApplicativeFamily for OptionType {
    fn pure<T>(inner: T) -> Self::Type<T> {
        Some(inner)
    }

    fn apply<T, U, F>(value: Self::Type<T>, f: Self::Type<F>) -> Self::Type<U>
    where
        F: FnMut(T) -> U,
    {
        value.zip(f).map(|(v, mut f)| f(v))
    }
}

struct MyOptionType {
}

impl MyOptionType {
    fn new<T>(inner: T) -> Option<T> {
        Some(inner)
    }
}


struct MyOption<T: ApplicativeFamily> {
    str: T::Type<String>,
    int: T::Type<i32>
}

impl <T> MyOption<T>
where
    T: ApplicativeFamily 
{
    fn hello(&self) -> u64 {
        let a = T::pure(123);
        let b = T::pure(String::from("123"));
//        let a = OptionType::fmap(self.str.clone(), |a| 123_u64);

        a.unwrap()
    }
}

trait Iterable {
    type Item<'a>
    where 
        Self: 'a;
    type Iterator<'a>: Iterator<Item = Self::Item<'a>>
    where 
        Self: 'a;

    fn iter<'c>(&'c self) -> Self::Iterator<'c>;
}

impl<T> Iterable for Vec<T> {
    type Item<'a> = &'a T
    where 
        Self: 'a;

    type Iterator<'a> = Iter<'a, T>
    where 
        Self: 'a;

    fn iter<'c>(&'c self) -> Self::Iterator<'c> {
        Iter{data: self}
    }
}


fn main() {
    println!("Hello, world!");
    let a: MyStruct<RcPointer> = MyStruct {
        num: Rc::new(1_i32),
        str: Rc::new(String::from("hello")),
    };


    let opt: MyOption<OptionType>= MyOption{
        str: OptionType::new(String::from("aaaa")),
        int: OptionType::new(123),
    };
}
