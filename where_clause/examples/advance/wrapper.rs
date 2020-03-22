// suppose we have a wrapper type
struct Wrapper<T> where T: std::fmt::Debug {
    pub inner: T
}

// because Wrapper take T as generic type, so the method that
// take Wrapper as parameter should also fulfil the constraint that
// T must implement std::fmt::Debug, but actually we don't care
fn take_wrapper<T>(w: Wrapper<T>) -> T
    where T:std::fmt::Debug {
    w.inner
}


// so we can use later bound the solve this problem
// we remove the where clause on struct and make inner private
pub struct LaterBoundWrapper<I> {
    inner: I
}

impl <I> LaterBoundWrapper<I>  {
    // and we add constraint on new method, that means, whenever we want a Wrapper
    // we have to create it through our factory method, and that will make sure
    // only object that implement std::fmt::Debug can be used 
    pub fn new(d: I) -> Self 
        where I: std::fmt::Debug
    {
        LaterBoundWrapper {
            inner: d
        }
    }

    // we have to add an inspect function here
    pub fn inspect(&self) where I: std::fmt::Debug {
        println!("inspect inner {:?}", self.inner);
    }
} 

// we can remove the constraint on the take_later_bound_wrapper now
pub fn take_later_bound_wrapper<T>(w: LaterBoundWrapper<T>) -> T {
    // we can't debug here, as we don't know if inner is Debug
    // println!("{:?}", w.inner);
    w.inner
}
