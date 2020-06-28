use std::pin::Pin;
use std::ptr::NonNull;

fn main() {
    println!("Hello, world!");
}

pub struct Connection;

pub struct UserRepository(NonNull<Connection>);

pub struct AppState {
    conn: Connection,
    user_repo: UserRepository
}

impl AppState {
    pub fn new(conn: Connection) -> Pin<Box<Self>> {
        let res = Self {
            conn,
            user_repo: UserRepository(NonNull::dangling()),
        };

        let mut boxed = Box::pin(res);
        let ref1 = NonNull::from(&boxed.conn);

        let mut_ref: Pin<&mut Self> = Pin::as_mut(&mut boxed);
        Pin::get_mut(mut_ref).user_repo = UserRepository(ref1);
        boxed

    }
}
