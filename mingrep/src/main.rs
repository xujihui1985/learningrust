extern crate mingrep;
use std::env;
use std::process;

use mingrep::Config;

fn main() {
    let args:Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        // eprintln will write message to stderr
        eprintln!("Failed to parse arguments: {}", err);
        process::exit(1);
    });

    mingrep::run(config).unwrap_or_else(|err| {
        eprintln!("Failed to process, {}", err);
        process::exit(1);
    });
}
