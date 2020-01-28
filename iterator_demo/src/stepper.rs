use std::ops::AddAssign;

pub struct Stepper<T> {
    curr: T,
    step: T,
    stop: T,
}

impl<T> Stepper<T> {
    pub fn new(start: T, stop: T, step: T) -> Self {
        Stepper {
            curr: start,
            stop,
            step,
        }
    }
}

impl<T> Iterator for Stepper<T>
    where T: AddAssign + Copy + PartialOrd
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.curr >= self.stop {
            return None;
        }
        let res = self.curr;
        self.curr += self.step;
        Some(res)
    }
}

fn sum_list<I, S>(l: I, mut s: S) -> S
    where I: Iterator<Item=S>,
          S: AddAssign,
{
    let mut it = l.into_iter();
    while let Some(n) = it.next() {
        s += n;
    }
//    for n in l {
//        s += n;
//    }
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut c = 0;
        let s = Stepper {
            curr: 2,
            stop: 10,
            step: 2,
        };
        for n in s {
            c += n;
        }
        assert_eq!(c, 20);

        let sum = sum_list(Stepper::new(2, 10 ,2), 0);
        assert_eq!(sum, 20);
    }
}