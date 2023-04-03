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

    let mut my_vec: Vec<&i32> = vec![];
    let v = vec![1,2,3];
    for i in &v {
        insert_value(&mut my_vec, i);
    }
    
    
    let text = String::from("Twas brillig, and the slithy toves // Did gyre and gimble in the wabe: // All mimsy were the borogoves, // And the mome raths outgrabe. ");
    let mut word_iterator = WordIterator::new(&text);
    let a = word_iterator.next_word();
    let b = word_iterator.next_word();
    assert_eq!(a, Some("aaa"));
    //assert_eq!(word_iterator.next_word(), Some("Twas"));
    //assert_eq!(word_iterator.next_word(), Some("brillig,"));
}

fn insert_value<'vec, 'content>(my_vec: &'vec mut Vec<&'content i32>, value: &'content i32) {
    my_vec.push(value)
}

#[derive(Debug)]
struct WordIterator<'s> {
    position: usize,
    string: &'s str
}

impl<'lifetime> WordIterator<'lifetime> {
    /// Creates a new WordIterator based on a string.
    fn new(string: &'lifetime str) -> WordIterator<'lifetime> {
        WordIterator {
            position: 0,
            string
        }
    }
    
    /// Gives the next word. `None` if there aren't any words left.
    fn next_word(&mut self) -> Option<&'lifetime str> {
        let start_of_word = &self.string[self.position..];
        let index_of_next_space = start_of_word.find(' ').unwrap_or(start_of_word.len());
        if start_of_word.len() != 0 {
            self.position += index_of_next_space + 1;
            Some(&start_of_word[..index_of_next_space]) 
        } else {
            None
        }
    }

}

struct StrWrap<'a>(&'a str);

fn make_wrapper(string: &str) -> StrWrap {
    StrWrap(string)
}

struct Ref<'a, T>(&'a T);


