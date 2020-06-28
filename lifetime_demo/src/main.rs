mod gbp;
mod lifetime;

fn announce<'a, T>(value: &'a T) 
    where T: std::fmt::Display {

        println!("{}", value);
}

fn evil_feeder<T>(input: &mut T, val: T) {
        *input = val;
}

struct Context<'a>(&'a str);

struct Parser<'a, 's> {
    context: &'a Context<'s>
}

impl<'a, 's> Parser<'a, 's> {
    fn parse(&self) -> Result<(), &'s str> {
        Err("aaa")
    }
}

fn parse_context(context: Context) -> Result<(), &str> {
    let p = Parser {context: &context};
    p.parse()
}

static LIST:[i32;4] = [1,2,3,4];
fn return_first_two_with_static() -> &'static [i32] {
    &LIST[0..2]
}

// struct Object {
//     number: u32
// }

// struct Multipiler {
//     object: &Object,
//     mult: u32
// }

// fn print_borrower_number(mu: Multipiler) {
//     println!("result: {}", mu.object.number * mu.mult);
// }

struct RefObject<'x>(&'x u32);

fn steal_a_var(o: RefObject) {
    println!("{}", o.0);
}

fn main() {

    let num = 42;
    let num_ref = &num;
    announce(num_ref);

    let mut mr_snuggles: &str = "meow! :3";  // mr. snuggles forever!!
    let spike = String::from("bark! >:V");
    let spike_str: &str = &spike;                // Only lives for the block
    evil_feeder(&mut mr_snuggles, spike_str);    // EVIL!
   //  println!("{}", mr_snuggles);           

    let result: &i32;
    {
        let x = 42 + 42;
        result = &x;
        println!("result is {}", result);
    }

    // let obj = Object {
    //     number: 5,    
    // };

    // let obj3 = Multipiler {
    //     object: &obj,
    //     mult: 3
    // };

    // print_borrower_number(obj3);

    let a = 3;
    let mut b = &a;

    let c = RefObject(&b);

    steal_a_var(c);

    let d = &mut b;

    
    lifetime::lifetime_test();
}

