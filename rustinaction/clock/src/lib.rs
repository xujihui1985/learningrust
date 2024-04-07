use chrono::{Local, DateTime};

struct Clock;

impl Clock {
    fn get() -> DateTime<Local> {
        Local::now()
    }

    fn set() -> ! {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use crate::Clock;


    #[test]
    fn it_works() {
        let now = Clock::get();
        println!("{}",now.timestamp());
        println!("{}",now.to_rfc2822());
        println!("{}",now.to_rfc3339());
    }
}
