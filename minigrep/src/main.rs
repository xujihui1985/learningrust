use std::env;
use std::process;
use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("failed to parse args {}", err);
        process::exit(1);
    });

    println!("query is {}", config.query);
    println!("filename is {}", config.filename);
    if let Err(err) = minigrep::run(config) {
        eprintln!("application error: {}", err);
        process::exit(1);
    }
//    run(config).unwrap_or_else(|err| {
//        println!("application error: {}", err);
//        process::exit(1);
//    });
}


