static url: &'static str = "aaaa";

trait Stores<TModel>:Storage{
    fn get_by_id(&self, id: u64) -> TModel;
}

trait Storage {
    fn query<TModel>(&self, id: u64) -> TModel 
        where 
            Self: Stores<TModel> // Self constrain
    {
        self.get_by_id(id)
    }
}

struct UserDatabase {}

impl Storage  for UserDatabase {}
impl Stores<User> for UserDatabase {
    fn get_by_id(&self, id: u64) -> User {
        User{id: 123}
    }
}
impl <'a> Stores<Avatar<'a>> for UserDatabase {
    fn get_by_id(&self, id: u64) -> Avatar<'a> {
        Avatar{id: 123, url: url}
    }
}


#[derive(Debug)]
struct User {
    id: u64,
}

#[derive(Debug)]
struct Avatar<'a> {
    id: u64,
    url: &'a str,
}


fn main() {
    let db = UserDatabase{};
    let u = db.query::<User>(1);
    println!("user is {:?}", u);
    let a = db.query::<Avatar>(2);
    println!("Avatar is {:?}", a);
}
