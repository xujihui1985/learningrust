use std::cell::{Ref, RefCell};
use std::rc::Rc;

type SingleLink = Option<Rc<RefCell<Node>>>;

#[derive(Clone)]
pub struct Node {
    value: String,
    next: SingleLink,
}

impl Node {
    pub fn new(value: &str) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node {
            value: value.to_owned(),
            next: None,
        }))
    }
}

pub struct TransactionLog {
    head: SingleLink,
    tail: SingleLink,
    pub length: u64,
}

impl TransactionLog {
    pub fn new_empty() -> TransactionLog {
        TransactionLog {
            head: None,
            tail: None,
            length: 0,
        }
    }

    pub fn append(&mut self, value: &str) {
        let new = Node::new(value);
        match self.tail.take() {
            Some(old) => old.borrow_mut().next = Some(new.clone()),
            None => self.head = Some(new.clone()),
        };
        self.length += 1;
        self.tail = Some(new);
    }

    pub fn pop(&mut self) -> Option<String> {
        self.head.take().map(|head| {
            if let Some(next) = head.borrow_mut().next.take() {
                self.head = Some(next);
            } else {
                self.tail.take();
            }
            self.length -= 1;
            let a = Rc::try_unwrap(head).ok().expect("");
            a.into_inner().value
        })
    }
}


