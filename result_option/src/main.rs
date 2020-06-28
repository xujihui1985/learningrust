fn main() {
    let v = Version{
        id: 123,
        published_by: Some(12),
    };
    let u = v.published_by();
    println!("{:?}", u);
}

#[derive(Debug)]
struct Version {
    id: i32,
    published_by: Option<i32>,
}

#[derive(Debug)]
struct User {
    id: i32,
    login: String,
}

impl User {
    fn find(id: i32) -> Self {
        User {
            id: 1,
            login: String::from("hello")
        }
    }
}

impl Version {
    pub fn published_by(&self) -> Option<User> {
        self.published_by.map(|p| User::find(p))
    }
}
