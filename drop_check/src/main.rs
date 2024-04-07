#![feature(dropck_eyepatch)]

pub struct Boks<T> {
    p: *mut T
}

// the compiler will assume droping the Boks will access the T
// dropping Boks, the compiler will assume it use T, if Boks implement drop
// regrardless the fact the drop implementation actually accesses the T, the 
// compiler assume that it accesses the T
unsafe impl<#[may_dangle] T> Drop for Boks<T> {
    fn drop(&mut self) {
        unsafe {Box::from_raw(self.p)};
    }
}

impl<T> Boks<T> {
    pub fn ny(t: T) -> Self {
        Boks {
            p: Box::into_raw(Box::new(t))
        }
    }
}

impl<T> std::ops::Deref for Boks<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        // Safety: is valid since it was constructed from a valid T, and turned into a pointer
        // through Box which creates aligned pointers, and hasn't been freed, since self is alive
        unsafe {&*self.p}
    }
}

impl<T> std::ops::DerefMut for Boks<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe {&mut *self.p}
    }
}

fn main() {
    let x = 32;
    let b = Boks::ny(x);
    println!("{:?}", *b);

    let mut y = 42;
    let b = Boks::ny(&mut y);
    println!("{:?}", y); // the drop implementation might access the inner value

}
