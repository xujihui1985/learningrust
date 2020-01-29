use std::cell::RefCell;
use std::rc::Rc;

type Link = Option<Rc<RefCell<Node>>>;

struct Node {
    next: Vec<Link>,
    pub offset: u64,
    pub command: String,
}

impl Node {
    pub fn new(v: Vec<Link>, offset: u64, command: &str) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node {
            next: v,
            offset,
            command: command.into(),
        }))
    }
}

struct BestTractionLog {
    head: Link,
    tails: Vec<Link>,
    max_level: usize,
    pub length: u64,
}

impl BestTractionLog {
    pub fn append(&mut self, offset: u64, value: &str) {
        let level = 1 + if self.head.is_none() {
            self.max_level
        } else {
            self.get_level()
        };

        let new = Node::new(vec![None; level], offset, value);
        for i in 0..level {
            if let Some(old) = self.tails[i].take() {
                let next = &mut old.borrow_mut().next;
                next[i] = Some(new.clone());
            }
            self.tails[i] = Some(new.clone());
        }

        if self.head.is_none() {
            self.head = Some(new.clone());
        }
        self.length += 1;
    }

    pub fn find(&self, offset: u64) -> Option<String> {
        match &self.head {
            Some(head) => {
                let mut start_level = self.max_level;
                let node = head.clone();
                let mut result = None;
                loop {
                    if node.borrow().next[start_level].is_some() {
                        break;
                    }
                    start_level -= 1;
                }
                let mut n = node;
                for level in (0..=start_level).rev() {
                    loop {
                        let next = n.clone();
                        match &next.borrow().next[level] {
                            Some(next) if next.borrow().offset <= offset => {
                                n = next.clone();
                            },
                            _ => break,
                        };
                    }
                    if n.borrow().offset == offset {
                        let tmp = n.borrow();
                        result = Some(tmp.command.clone());
                        break;
                    }
                }
                result
            }
            None => None
        }
    }

    fn get_level(&self) -> usize {
        let mut n = 0;
        while rand::random::<bool>() && n < self.max_level {
            n += 1;
        }
        n
    }
}