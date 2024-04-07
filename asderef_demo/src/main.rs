const DEFAULT_NAME: &str = "hello";

struct Config {
    name: Option<String>
}

impl Config {
    fn new() -> Self {
        Config { name: None }
    }
    fn get_name(&self) -> &str {
        self.name.as_deref().unwrap_or(DEFAULT_NAME)
    }
}

fn main() {
    let c = Config::new();
    let name = c.get_name();
    println!("name is {}", name)
}
