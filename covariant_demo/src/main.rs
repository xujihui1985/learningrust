use std::env;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect::<Vec<String>>();
    println!("args is {:?}", args);
    //let bbb = String::from("hello");
    //let mut bbb_str :&str = &bbb;
    //let mr_snuggles: &'static str = "hello";
    //let spike: &'static str = "bark";
    //let spike_str: &str = &spike;
    //evil_feeder(&mut bbb_str, spike_str);
    //evil_feeder(&mut mr_snuggles, spike_str);
    //println!("{}", mr_snuggles);
}

fn evil_feeder<T>(input: &mut T, val: T) {
    *input = val;
}
