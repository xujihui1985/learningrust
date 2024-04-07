use std::{collections::HashMap};

#[derive(Clone)]
enum Command {
    SetVar(String, Value),
    GetVar(String)
}

#[derive(Clone, Debug, PartialEq)]
enum Value {
    Nothing,
    Int(i64)
}

struct Evaluator {
    vars: HashMap<String, Value>,
}


#[derive(Debug)]
enum ParseError {
    MissingVariable(String),
}

#[derive(Debug)]
enum EvalError {
    MissingVariable(String),
}

impl Evaluator {
    fn new() -> Self {
        Evaluator {
            vars: HashMap::new()
        }
    }
    fn evaluate(&mut self, commands: &[Command]) -> Result<Value, EvalError> {
        let mut output = Ok(Value::Nothing);
        for command in commands {
            match command {
                Command::SetVar(name, value) => {
                    self.vars.insert(name.into(), value.clone());
                },
                Command::GetVar(name) => {
                    match self.vars.get(name) {
                        Some(value) => {
                            output = Ok(value.clone())
                        }
                        None => {
                            return Err(EvalError::MissingVariable(name.into()));
                        }
                    }
                }
            }
        }        
        output
    }
}

fn parse(input: &str) -> Result<Vec<Command>, ParseError> {
    // set a 100
    // get a

    for line in input.lines() {
        let cmd:Vec<_> = line.split_ascii_whitespace().collect();
        match cmd.get(0) {
            Some(&x) if x == "set" => {
                
            }
            Some(&x ) if x == "get" => {

            }
            _ => { }
        }
    }
    Ok(vec![])
}


fn main() {
    println!("Hello, world!");
    
}

#[cfg(test)]
mod tests {
   
    use super::*;

    #[test]
    fn test1() -> Result<(), EvalError>{
        let commands = vec![
            Command::SetVar("a".to_string(), Value::Int(100)),
            Command::GetVar("a".to_string()),
        ];
        let mut evaluator = Evaluator::new(); 
        let res = evaluator.evaluate(&commands)?;
        println!("{:?}", res);
        assert_eq!(res, Value::Int(100));
        Ok(())
    }
}