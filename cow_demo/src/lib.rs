use std::borrow::Cow;

fn get_cow(s: Option<String>) -> Cow<'static, str> {
    if let Some(a) = s {
        //Cow::Borrowed(&a)
        a.into()
    } else {
        Cow::Borrowed("hello")
    }
}

fn use_str(s: &str) {
    println!("222222222222 {}", s);
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn it_works() {
        let s = String::from("aaaaaaaaaaaaaa");
        let c = get_cow(None);
        match c {
            Cow::Borrowed(_) => println!("borrowed"),
            Cow::Owned(_) => println!("owned"),
        };
        use_str(&c);
    }
}
