use std::collections::HashMap;

// type alias
type Table = HashMap<String, Vec<String>>;

struct Person {
    name: Option<String>,
    birth: i32,
}

struct Anime {
    name: &'static str,
    bechdel_pass: bool,
}


#[derive(Debug, Copy, Clone)]
struct Label {
    number: u32,
}

fn print(l: Label) {
    println!("{:?}", l);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut v = Vec::new();
        for i in 101..106 {
            v.push(i.to_string());
        }

        let fifth = v.pop().unwrap();
        assert_eq!(fifth, "105");

        let second = v.swap_remove(1);
        assert_eq!(second, "102");

        let third = std::mem::replace(&mut v[2], "replace".to_string());
        assert_eq!(third, "103");


        assert_eq!(v, vec!["101", "104", "replace"]);
    }

    #[test]
    fn take_option() {
        let mut composers = Vec::new();
        composers.push(Person{name: Some("sean".to_string()), birth: 30});

        let first_name = composers[0].name.take();
        assert_eq!(first_name, Some("sean".to_string()));
        assert_eq!(composers[0].name, None);
    }

    #[test]
    fn test_copy() {
        let l = Label{number: 3};
        print(l);
        println!("number is {}", l.number);
    }

    #[test]
    fn test_deref() {
        let aria = Anime{
            name: "sean",
            bechdel_pass: true
        };

        let anime_ref = &aria;
        assert_eq!(anime_ref.name, (*anime_ref).name);
    }

    #[test]
    fn test_factorial() {
        let fac = |n: usize| -> usize {
            (1..n+1).fold(1, |a,b| a * b)
        };
        let r = &fac(6);
        assert_eq!(r + &1009, 1729);
    }
}


