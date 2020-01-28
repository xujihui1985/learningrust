use model::User;

mod model;
mod session;

fn main() {
    let u = User::new(String::from("sean"), "passwd".to_string()).unwrap();
    println!("{:?}", u);
    let result = u.verify("passwd");
    println!("{}", result);
}
