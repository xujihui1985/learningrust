#[macro_export]
macro_rules! avec  {
    ($arg1:ty as $arg2:ident;) => {
       type $arg2 = $arg1;
    };
    () => {
        Vec::new()
    };
    // $($xxx:xxx),+ // match one or more list seprate by ,
    // $(,)? means allow trailing comma
    ($($element:expr),+ $(,)?) => {   // rust not allow `let` keyword to exist in macro, if we need let syntax, we need nest block
        {// nest block that return vs that make it a valid expression
            let mut vs = Vec::new();
            $(vs.push($element);)+ // repeat as many as time as $element pattern repeated
            vs
        }
    };
    ($element:expr; $count:expr) => {{
        let mut vs = Vec::new();
        let x = $element; // if element is something like Some(x).take().unwrap, it is not idenpodent to call twice
        // and remember macro_rules is just substution, it will panic if we call twice on $element expr
        for _ in 0..$count {
            vs.push(x.clone());
        }
    }};
}

pub trait MaxValue {
    fn max_value() -> Self;
}

#[macro_export]
macro_rules! max_impl {
    ($t:ty) => {
       impl $crate::avec::MaxValue for $t {
           fn max_value() -> Self {
               <$t>::MAX
           }
       } 
    };
}

pub(crate) use avec;
pub(crate) use max_impl;
