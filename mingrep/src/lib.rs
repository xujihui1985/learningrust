use std::error::Error;
use std::io::prelude::*;
use std::fs::File;
use std::{env, thread};
use std::time::Duration;

pub struct Config {
    query: String,
    filename: String,
    case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
        Ok(Config {query, filename, case_sensitive})
    }

}

pub fn run(config: Config) -> Result<(), Box<Error>>{
    let mut f = File::open(config.filename)?;
    let mut content = String::new();
    f.read_to_string(&mut content)?;
    for line in search(&config.query, &content) {
        println!("matched line {}", line);
    }
    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            result.push(line);
        }
    }
    let mut expensive_result = Cacher::new(|num| {
        println!("calcualing slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });
    println!("result of expensive is {}", expensive_result.value(12));
    println!("result of expensive 2 is {}", expensive_result.value(13));

    result
}

fn clousure() {
    let expensive_clousure = Cacher::new(|num| {
        println!("calcualing slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });
}

struct Cacher<T>
    where T:Fn(u32) -> u32
{
    calculation: T,
    value: Option<u32>,
}

impl<T> Cacher<T>
    where T: Fn(u32) -> u32
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            },
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn on_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents))
    }

    #[test]
    fn iterator_demo() {
        let v1 = vec![1,2,3];
        let mut v1_iter = v1.iter();
        assert_eq!(v1_iter.next(), Some(&1));

    }
}