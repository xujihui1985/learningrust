pub enum Result<T, E> {
    Ok(T),
    Err(E),
}

impl <T, E> Result<T, E> 
    where E: std::fmt::Debug {
    pub fn unwrap(self) -> T {
        match self {
            Result::Ok(t) => t,
            Result::Err(e) => panic!("error: {:?}", e),
        }
    }
}

impl <T, E> Result<T, E> 
    where T: Default {

    pub fn unwrap_or_default(self) -> T {
        //
    }
}

