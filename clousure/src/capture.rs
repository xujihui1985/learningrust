pub fn inc_by(num: u32) -> impl Fn(u32) -> u32 {
    move |mut v| {
        v += num;
        v
    }
}

pub fn prefix<'a>(s: &'a str) -> impl Fn(&str) -> String + 'a {
    move |v| {
        format!("{} {}", s, v)
    }
}

fn capture_environment() -> bool {
    let x = 4;
    let equal_to_x = |z| z == x;
    let y = 4;
    equal_to_x(y)
}

fn move_variable() -> bool {
    let x = vec![1, 2, 3];
    let equal_to_x: Box<dyn FnOnce(Vec<u32>) -> bool> = Box::new(move |z| z == x);

//    println!("{:?}", x); // this won't compile, because x has been moved, so equal_to_x is FnOnce
    let y = vec![1, 2, 3];
    equal_to_x(y)
}

fn mutate_environment() -> Vec<i32> {
    let mut x = vec![1, 2, 3];
    let mutate_x = &mut |z| x.iter_mut().for_each(|y| *y = *y + z);
    mutate_x(1);
    println!("{:?}", x);
    x
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inc_by_1() {
        let f = inc_by(1);
        let num = 10;
        f(num);
        assert_eq!(10, num)
    }

    #[test]
    fn equal_to_x() {
        let res = capture_environment();
        assert_eq!(res, true);
    }

    #[test]
    fn equal_to_vec() {
        let res = move_variable();
        assert_eq!(res, true);
    }

    #[test]
    fn mutate_clousure() {
        let res = mutate_environment();
        assert_eq!(res, vec![2, 3, 4])
    }

    #[test]
    fn prefix_str() {
        let res = prefix("hello")("aaaaaaa");
        assert_eq!(res, "hello aaaaaaa");
    }
}