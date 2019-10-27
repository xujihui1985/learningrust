use std::error::Error;
use std::fmt;

#[derive(Debug)]
enum OperationsError {
    DivideByZeroError,
}

macro_rules! fractorial {
    ($x:expr) => {
        {
            let mut result = 1;
            for i in 1..($x+1) {
                result = result * i;
            }
            result
        }
    };
}

impl fmt::Display for OperationsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            OperationsError::DivideByZeroError => f.write_str("cannot divide by zero")
        }
    }
}

impl Error for OperationsError {
    fn description(&self) -> &str {
        match *self {
            OperationsError::DivideByZeroError => "can not divide by zero"
        }
    }
}

fn divide(dividend: u32, divisor: u32) -> Result<u32, OperationsError> {
    if divisor == 0u32 {
        Err(OperationsError::DivideByZeroError)
    } else {
        Ok(dividend / divisor)
    }
}

fn main() {
    let result1 = divide(1, 0);
    match result1 {
        Ok(res) => println!("result is {}", res),
        Err(e) => println!("error is {}", e)
    }
    let res = fractorial!(2);
    println!("{}", res);

    let sum =
        (0..).map(|x| x * x)
            .take_while(|&x| x < 500)
            .filter(|x| *x % 2 == 0)
            .fold(0, |sum, x| sum + x);

    println!("sum is {}", sum);

    let v: Vec<i32> = vec![1, 2, 3, 4, 5];
    let v2 = v
        .iter()
        .map(|x| x + 1)
        .filter(|&x| x < 6)
        .fold(0, |sum, cur| sum + cur);
    println!("v2 is {:?}", v2);
}


