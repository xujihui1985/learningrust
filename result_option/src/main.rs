fn main() {
    //let v = Version{
        //id: 123,
        //published_by: Some(12),
    //};
    //let u = v.published_by();
    //println!("{:?}", u);

    let mut w = Wrapper(Some(Data));
    let d = w.get_data_ref();
    let res = d.as_ref().map(|data| data.val());

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

#[derive(Debug)]
struct Data;

#[derive(Debug)]
struct Wrapper(Option<Box<Data>>);

impl Data {
    fn val(&self) -> u32 {
        10
    }
}

impl Wrapper {
    fn get_data(&self) -> Option<&Data> {
        self.0.as_deref()
    }


    fn get_data_ref(&self) -> &Option<Box<Data>> {
        &self.0
    }

}


