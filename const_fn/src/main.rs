use std::sync::Mutex;

struct WillSayGoodbye<'a>(&'a str);

impl<'a> Drop for WillSayGoodbye<'a> {
    fn drop(&mut self) {
        println!("{}", self.0);
    }
}

// this running at compile time
const _:() = { 

    () 
};

const fn five_numbers() -> [i32; 5] {
    let mut numbers = [0_i32; 5];
    
    let mut i = 0;
    while i < 5 {
        numbers[i] = i as i32 + 1;
        i += 1;
    }
    //for i in 0..5 {
    //}
    numbers
}

const fn numbers<const N:usize>() -> [i32; N] {
    let mut numbers = [0_i32; N];
    
    let mut i = 0;
    while i < N {
        numbers[i] = i as i32 + 1;
        i += 1;
    }
    //for i in 0..5 {
    //}
    numbers
}

const fn len(strs: &[&str]) -> usize {
    let mut result = 0;
    let mut remaining = strs;

    while let [current, tail@..] = remaining {
        result += current.len();
        remaining = tail;
    }
    result
}

struct Buf<const N:usize>([u8; N]);

const fn concat<const N:usize>(strs: &[&str]) -> Buf<N> {
    let mut buffer = [0; N];
    let mut position_in_buffer = 0;

    let mut remaining = strs;
    while let [current, tail@..] = remaining {
        let x = current.as_bytes();
        let mut i = 0;

        // cannot use copy_from_slice because mutable reference are not allowed
        // in const functions. buffer.copy_from_slice(x);
        // and for loop are not allowed in const functions yet
        while i < x.len() {
            buffer[position_in_buffer] = x[i];
            position_in_buffer += 1;
            i+=1;
        }
        remaining = tail;
    }
    Buf(buffer)
}

#[derive(Copy, Debug, Clone)]
struct Customer<'a> {
    name: &'a str,
    age: i32,
}

const CUSTOMER: Customer<'static> = Customer {
    name: "hello",
    age: 12,
};

const fn nth<T: Copy, const N: usize>(item: [T; N], index: usize) -> T{
    item[index]
}


trait Animal {
    fn make_sound<'a>(&self) -> &'a str;
}

struct Cat;
struct Dog;

impl Animal for Cat {
    fn make_sound<'a>(&self) -> &'a str {
        "meow"
    }
}

impl Animal for Dog {
    fn make_sound<'a>(&self) -> &'a str {
        "woof"
    }
}

const fn favorite_animal() -> impl Animal {
    Cat
}

const fn animal_by_sound<'a>(can_purr: bool) -> &'a dyn Animal {
    match can_purr {
        true => &Cat,
        false => &Dog,
    }
}

fn return_ref<'a>() -> &'a Cat {
    &Cat
}

static ARRAY: Mutex<Vec<u8>> = Mutex::new(vec![]);


fn main() {
    const FIVE_NUMBERS: [i32; 5] = five_numbers();

    let five_numbers = five_numbers();

    const TEN_NUMBERS: [i32; 10] = numbers();

    const CUSTOMERS: [Customer; 2] = [
        Customer {
            name: "aaa",
            age: 21,
        },
        CUSTOMER,
    ];

    const NTH_CUST: Customer = nth(CUSTOMERS, 1);

    println!("{:?}", NTH_CUST);


    const MYCAT: &dyn Animal = &favorite_animal();

    let a = animal_by_sound(true);
    println!("{}", a.make_sound());

    {
    let mut arr = ARRAY.lock().unwrap();
    arr.push(1);
    }

}
