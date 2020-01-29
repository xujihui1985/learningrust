use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct Node {
    value: String,
    next: Link,
    prev: Link,
}

impl Node {
    pub fn new(val: &str) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node {
            value: val.into(),
            next: None,
            prev: None,
        }))
    }
}

type Link = Option<Rc<RefCell<Node>>>;

pub struct BetterTransactionLog {
    head: Link,
    tail: Link,
    pub length: u64,
}

impl BetterTransactionLog {
    pub fn new() -> BetterTransactionLog {
        BetterTransactionLog {
            head: None,
            tail: None,
            length: 0,
        }
    }

    pub fn append(&mut self, val: &str) {
        let new = Node::new(val);
        match self.tail.take() {
            Some(old) => {
                old.borrow_mut().next = Some(new.clone());
                new.borrow_mut().prev = Some(old);
            },
            None => self.head = Some(new.clone()),
        };
        self.length += 1;
        self.tail = Some(new);
    }

    pub fn iter(&self) -> ListIterator {
        ListIterator {
            current: self.head.clone()
        }
    }
}

pub struct ListIterator {
    current: Link,
}

impl ListIterator {
    pub fn new(start: Link) -> ListIterator {
        ListIterator {
            current: start,
        }
    }
}

impl DoubleEndedIterator for ListIterator {
    fn next_back(&mut self) -> Option<Self::Item> {
        let current = &self.current;
        let mut result = None;
        self.current = match current {
            Some(current) => {
                let current = current.borrow();
                result = Some(current.value.clone());
                current.prev.clone()
            }
            None => None,
        };
        result
    }
}

impl Iterator for ListIterator {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let current = &self.current;
        let mut result = None;
        self.current = match current {
            Some(current) => {
                let current = current.borrow();
                result = Some(current.value.clone());
                current.next.clone()
            }
            None => None
        };
        result
    }
}
