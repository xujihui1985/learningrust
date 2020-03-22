// the function take any args which can be turn into String
pub fn takes_into_string<T>(t: T) 
    where String: From<T> {
    let str = String::from(t);
    println!("str is {}", str);
}
