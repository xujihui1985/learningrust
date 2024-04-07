use std::collections::binary_heap::Iter;

pub struct Flatten<O>
where
    O: Iterator,
    O::Item: IntoIterator,
{
    outer: O,
    front_iter: Option<<O::Item as IntoIterator>::IntoIter>,
    back_iter: Option<<O::Item as IntoIterator>::IntoIter>,
}

impl<O> Flatten<O>
where
    O: Iterator,
    O::Item: IntoIterator,
{
    fn new(iter: O) -> Self {
        Flatten {
            outer: iter,
            front_iter: None,
            back_iter: None,
        }
    }
}

impl<O> Iterator for Flatten<O>
where
    O: Iterator,
    O::Item: IntoIterator,
{
    type Item = <O::Item as IntoIterator>::Item; // full qulified name required

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(ref mut front_iter) = self.front_iter {
                if let Some(i) = front_iter.next() {
                    return Some(i);
                }
                let _ = self.front_iter.take();
            }
            if let Some(next_inner) = self.outer.next() {
                self.front_iter = Some(next_inner.into_iter());
            } else {
                return self.back_iter.as_mut()?.next(); // as_mut: convert Option<T> -> Option<&mut T>
            }
        }
    }
}

impl<O> DoubleEndedIterator for Flatten<O>
where
    O: Iterator + DoubleEndedIterator, // DoubleEndedIterator is also Iterator
    O::Item: IntoIterator,
    <O::Item as IntoIterator>::IntoIter: DoubleEndedIterator, // the assosiated type of Item
{
    fn next_back(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(ref mut back_iter) = self.back_iter {
                if let Some(i) = back_iter.next_back() {
                    return Some(i);
                }
                let _ = self.back_iter.take();
            }
            if let Some(next_inner) = self.outer.next_back() {
                self.back_iter = Some(next_inner.into_iter());
            } else {
                return self.front_iter.as_mut()?.next_back();
            }
        }
    }
}

pub fn flatten<I>(iter: I) -> Flatten<I>
where
    I: Iterator,
    I::Item: IntoIterator,
{
    Flatten::new(iter)
}

pub trait IteratorExt: Iterator {
    fn out_flatten(self) -> Flatten<Self>
    where
        Self: Sized,
        Self::Item: IntoIterator;
}

impl<T> IteratorExt for T
where
    T: Iterator,
{
    fn out_flatten(self) -> Flatten<Self>
    where
        Self::Item: IntoIterator,
    {
        flatten(self)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn it_works() {
        let i = flatten(vec![vec![1, 2, 3], vec![3, 4]].iter()).count();
        assert_eq!(5, i);
    }

    #[test]
    fn reverse() {
        /*
         fn rev(self) -> Rev<Self>
        where
            Self: Sized + DoubleEndedIterator,
        {
                Rev::new(self)
        }
         */
        let res = flatten(vec![vec![1, 2, 3], vec![3, 4]].into_iter())
            .rev()
            .collect::<Vec<_>>();
        //rev only exists when Self implement  DoubleEndedIterator trait
        assert_eq!(vec![4, 3, 3, 2, 1], res);
    }

    #[test]
    fn ext() {
        vec![vec![1, 2, 3], vec![3, 4]].iter().out_flatten();
    }
}
