fn describe_point(x: i32, y: i32) -> &'static str {
    use std::cmp::Ordering::{Equal, Greater, Less};
    match (x.cmp(&0), y.cmp(&0)) {
        (Equal, Equal) => "at the origin",
        (_, Equal) => "on the x axis",
        (Equal, _) => "on the y axis",
        (Greater, Greater) => "in the first quadrant",
        (Less, Greater) => "in the second quadrant",
        _ => "somewhere else"
    }
}

#[derive(Debug)]
struct Account {
    id: u32,
    name: String,
    language: String,
    status: String,
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn match_struct(account: &Account) -> String {
    match account {
        Account {
            name,
            language,
            ..
        } => {
            format!("{}, {}", name, language)
        }
    }
}

fn range_match(next_char: char) {
    // notice ... are inclusive while .. are exclusive
    match next_char {
        '0'...'9' => {
            println!("it is a number");
        }
        'a'...'z' | 'A'...'Z' => {
            println!("it is a word");
        }
        ' ' | '\t' | '\n' => {
            println!("it is a space");
        }
        _ => println!("invalid")
    }
}

fn show_account(account: &Account) -> String {
    format!("{}, {}, {}", account.status, account.name, account.language)
}

fn get_location(x: i32, y: i32) -> Option<Point> {
    Some(Point { x, y })
}

fn pattern_guards() {
    // if point is not copyable, it must be a ref
    match get_location(10, 20) {
        Some(ref point) if point.x < 20 => {
            println!("x is below 20")
        }
        Some(point) => {
            println!("x is above 10")
        }
        None => println!("invalid")
    }
}

fn total_match_patten() {
    match get_location(10, 20) {
        Some(pot@Point{x:10,..}) => {
            println!("pot is {}:{}", pot.x, pot.y)
        }
        Some(point) => {
            println!("x is above 10")
        }
        None => println!("invalid")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_describe_point() {
        let res = describe_point(10, 20);
        assert_eq!("in the first quadrant", res);
    }

    #[test]
    fn test_match_struct() {
        let account = Account {
            id: 123,
            name: String::from("Sean"),
            language: String::from("Chinese"),
            status: String::from("activate"),
        };
        let res = match_struct(&account);
        println!("{:?}", account);
        assert_eq!("Sean, Chinese", res);
    }

    #[test]
    fn test_ref_match() {
        let account = Account {
            id: 123,
            name: String::from("Sean"),
            language: String::from("Chinese"),
            status: String::from("activate"),
        };
        match account {
            Account { ref name, ref language, .. } => {
                show_account(&account);
            }
        }
    }

    #[test]
    fn test_range() {
        range_match('a');
    }

    #[test]
    fn test_pattern_guard() {
        pattern_guards();
    }

    #[test]
    fn test_total_match_patten() {
       total_match_patten();
    }
}
