struct Person<'a> {
    name: &'a str,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let name = "hello";
        let b = &name;
        let p = Person {
            name: b,
        };
    }
}
