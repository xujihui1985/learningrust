mod custom_error;

fn main() -> Result<(), Box<dyn std::error::Error>>{
    let l = Hello{a: 123};
    forget_sth(l);
    println!("Hello, world!");
    Ok(())
}

fn save_status(text: &str) -> Result<i64, &'static str> {
    if text.len() > 200 {
        return Err("aaaaaaaaaa");
    }
    let record = save_to_database(text)?;

    Ok(record.id)
}

fn save_to_database(text: &str) -> Result<StatusRecord, &'static str> {
    Err("aaaaa")
}

struct StatusRecord {
    id: i64,
    text: String,
    created_at: std::time::Instant,
}

fn add_five(n: &str) -> Result<i32, std::num::ParseIntError> {
    let num: i32 = n.parse()?;
    Ok(num + 5)
}

struct Hello {
    a: u32,
}

impl Drop for Hello {
    fn drop(&mut self) {
        println!("drop");
    }
}

fn forget_sth(a: Hello) {
    println!("inside forget_sth");
    std::mem::forget(a);
}