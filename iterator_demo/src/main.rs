use std::iter::{Peekable, repeat, once};
mod stepper;
mod flatten;

fn main() {
    let res = (1..10).scan(10, |s, x| {
        *s = *s + x;
        println!("{} {}", *s, x);
        Some(*s)
    }).collect::<Vec<_>>();
    println!("{:?}", res);

    let text = "hello \n world\n bbb\n ccccccc".to_string();
    let v: Vec<&str> = text.lines()
        .map(str::trim)
        .filter(|s| *s != "bbb")
        .collect();
    assert_eq!(v, ["hello", "world", "ccccccc"]);

    let iter = (0..10)
        .scan(0, |sum, item| {
            *sum = *sum + item;
            if *sum > 10 {
                None
            } else {
                Some(item * item)
            }
        });

    let res: Vec<i32> = iter.collect();
    assert_eq!(res, vec![0, 1, 4, 9, 16]);

    let message = "To: jimb\r\n\
               From: superego <editor@oreilly.com>\r\n\
               \r\n\
               Did you get any writing done today?\r\n\
               When will you stop wasting time plotting fractals?\r\n";
    for header in message.lines().inspect(|l| println!("{}", l)).take_while(|l| !(*l).is_empty()) {
        println!("{}", header);
    }

    let mut chars = "51111133,3332111".chars().peekable();
    let r = parse_number(&mut chars);
    println!("result is {}", r);


    let message2 = "To: jimb\r\n\
               From: id\r\n\
               \r\n\
               Oooooh, donuts!!\r\n";

    let mut lines = message2.lines();
    for headers in lines.by_ref().take(3) {
        println!("header is {}", headers);
    }

    println!("\n Body:");
    for body in lines {
        println!("{}", body);
    }

    let fizzes = repeat("").take(2).chain(once("fizz")).cycle();
    let buzzes = repeat("").take(4).chain(once("buzz")).cycle();
    let fizzes_buzzes = fizzes.zip(buzzes);
//    let fizz_buzz = (1..100).zip(fizzes_buzzes)
//        .map(|tp| {
//            match tp {
//                (i, ("", "")) => i.to_string(),
//                (_, (fizz, buzz)) => format!("{}{}", fizz, buzz)
//            }
//        });
//
    for line in repeat(" ").take(2).chain(once("fizz")) {
        println!("{}", line);
    }
}

fn parse_number<I>(tokens: &mut Peekable<I>) -> u32
    where I: Iterator<Item=char> {
    let mut n = 0;
    loop {
        match tokens.peek() {
            Some(r) if r.is_digit(10) => {
                n = n * 10 + r.to_digit(10).unwrap();
            }
            _ => return n
        }
        tokens.next();
    }
}
