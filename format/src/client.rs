pub enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

pub fn connect() {
    println!("connect called");
}