use std::cell::{Cell, UnsafeCell};

fn main() {
    println!("Hello, world!");
}

pub struct MyCell<T> {
    value: UnsafeCell<T>,
}

// unsafe impl<T> !Sync for MyCell<T> {}

impl<T: Copy> MyCell<T> {
    pub fn new(value: T) -> Self {
        MyCell {
            value: UnsafeCell::new(value),
        }
    }

    pub fn set(&self, value: T) {
        // SAFETY
        unsafe { *self.value.get() = value };
    }

    pub fn get(&self) -> T {
        unsafe { *self.value.get() }
    }
}

#[derive(Copy, Clone)]
enum RefState {
    Unshared,
    Shared(usize),
    Exclusive,
}

pub struct MyRefCell<T> {
    value: UnsafeCell<T>,
    state: Cell<RefState>,
}

impl<T> MyRefCell<T> {
    pub fn new(value: T) -> Self {
        Self {
            value: UnsafeCell::new(value),
            state: Cell::new(RefState::Unshared),
        }
    }

    pub fn borrow(&self) -> Option<&T> {
        match self.state.get() {
            RefState::Unshared => {
                self.state.set(RefState::Shared(1));
                Some(unsafe { &*self.value.get() })
            }
            RefState::Shared(n) => {
                self.state.set(RefState::Shared(n + 1));
                Some(unsafe { &*self.value.get() })
            }
            RefState::Exclusive => None,
        }
    }

    pub fn borrow_mut(&self) -> Option<&mut T> {
        if let RefState::Unshared = self.state.get() {
            self.state.set(RefState::Exclusive);
            Some(unsafe { &mut *self.value.get() })
        } else {
            None
        }
    }
}

#[cfg(test)]
mod test {
    use std::{sync::Arc, thread};

    use super::MyCell;

    fn bad() {
        let x = Arc::new(MyCell::new(1));
        let x1 = Arc::clone(&x);
        thread::spawn(|| {
            x1.set(2);
        });
        let x2 = Arc::clone(&x);
        thread::spawn(|| {
            x2.set(3);
        });
    }
}
