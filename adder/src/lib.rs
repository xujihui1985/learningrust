#[derive(Debug)]
pub struct Rectangle{
    length: u32,
    width: u32,
}

impl Rectangle {
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.length > other.length && self.width > other.width
    }

    pub fn panic() {
        panic!("panic");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn larger_can_hold() {
        let larger = Rectangle {
            length: 8,
            width: 7
        };
        let smaller = Rectangle {
            length: 7,
            width: 6
        };
        assert_eq!(true, larger.can_hold(&smaller));
    }

    #[test]
    #[should_panic]
    fn should_panic() {
        Rectangle::panic();
    }
}
