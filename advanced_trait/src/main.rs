use std::collections::HashMap;

pub fn balanced<T>(input: T) -> bool
    where T: Into<String> {

        let mut stack = Vec::new();

        let mut matches = HashMap::new();
        matches.insert(')', '(');
        matches.insert(']', '[');
        matches.insert('}', '{');

        for c in input.into().chars() {
            match c {
                '(' | '[' | '{' => stack.push(c),
                ')' | ']' | '}' => {
                    let pre = stack.pop().unwrap();
                    match matches.get(&c) {
                        Some(p) => {
                            return &pre == p;
                        },
                        _ => unreachable!(),
                    }
                }
                _ => return false
            }

        }
        stack.len() == 0
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {

    use super::balanced;
    
    #[test]
    fn it_works() {
        assert_eq!(balanced("()"), true);
        assert_eq!(balanced("({})"), true);
        assert_eq!(balanced("({[]})"), true);
        assert_eq!(balanced("([)"), false);
    }

    #[test]
    fn option_deq() {
        let s: String = "hello".into();
        println!("{}", s);
        let a = Some(1); // 01
        let b = Some(2); // 10
        let c = a.and(b);
        assert_eq!(c, Some(2));
    }
}

