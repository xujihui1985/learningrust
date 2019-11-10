use std::path::Path;

fn main() {
    let hello = Hello {
        name: String::from("aaaaa"),
    };
    do_sth(&hello);
}

struct Hello {
    name: String,
}

impl AsRef<String> for Hello {
    fn as_ref(&self) -> &String {
        &self.name
    }
}

fn do_sth(param: impl AsRef<String>) {
    let str = param.as_ref();
    println!("do sth with {}", str);
}

fn do_sth2(param: impl AsRef<Path>) {}
