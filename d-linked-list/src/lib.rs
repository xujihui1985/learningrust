use std::{cell::RefCell, rc::{Rc, Weak}};

pub struct Node<T: Copy> {
    pub value: T,
    pub next: Option<Rc<RefCell<Node<T>>>>,
    pub prev: Option<Weak<RefCell<Node<T>>>>,
}

impl<T: Copy> Node<T> {
    pub fn new(value: T) -> Self {
        Node {
            value,
            next: None,
            prev: None,
        }
    }
}

impl<T: Copy> From<Node<T>> for Option<Rc<RefCell<Node<T>>>>{
    fn from(node: Node<T>) -> Self {
        Some(Rc::new(RefCell::new(node)))
    }
}

type NodePtr<T> = Rc<RefCell<Node<T>>>;

pub struct List<T: Copy> {
    head: Option<NodePtr<T>>,
    tail: Option<NodePtr<T>>,
}

impl<T:Copy> List<T> {
    pub fn new() -> Self{
        List {
            head: None,
            tail: None,
        }
    }

    pub fn push_back(&mut self, value: T) {
        let mut node = Node::new(value);

        match &mut self.tail.take() {
            None => {
                self.head = node.into();
                self.tail = self.head.clone();
            }
            Some(current_tail) => {
                node.prev = Some(Rc::downgrade(&current_tail)); //downgrade to weak reference
                self.tail = node.into();
                current_tail.borrow_mut().next = self.tail.clone();
            }
        }
    }

    pub fn pop_back(&mut self) -> Option<T> {
        match &mut self.tail.take() {
            None => None,
            Some(tail) => {
                let mut tail = tail.borrow_mut();
                let prev = tail.prev.take();
                match prev {
                    None => {
                        self.head.take();
                    },
                    Some(prev) => {
                        let prev = prev.upgrade();
                        if let Some(prev) = prev {
                            prev.borrow_mut().next = None;
                            self.tail = Some(prev);
                        }
                    }
                }
                Some(tail.value)
            }
        }
    }

    pub fn push_front(&mut self, value: T) {
        let mut node = Node::new(value);
        match &mut self.head.take() {
            None => {
               self.head = node.into();
               self.tail = self.head.clone();
            },
            Some(current_head) => {
                node.next = self.head.clone();
                self.head = node.into();
                if let Some(h) = &self.head {
                    current_head.borrow_mut().prev = Some(Rc::downgrade(&h));
                }
            },
        }
    }

    pub fn pop_front(&mut self) -> Option<T> {
        match &mut self.head.take() {
            None => None,
            Some(head) => {
                let mut head = head.borrow_mut();
                let next = head.next.take();
                match next {
                    None => {
                        self.tail.take();
                    },
                    Some(next) => {
                        next.borrow_mut().prev.take();
                        self.head = Some(next);
                    }
                } 
                Some(head.value)
            }
        }
    }
}

impl<T:Copy> Drop for List<T> {
    fn drop(&mut self) {
        while let Some(_) = self.pop_front() {}
    }
}

fn test2() {
   let r;//声明了没有初始值的变量,所以这些变量存在于外部 Scope 
    //这乍看之下好像和 Rust 不允许存在空值相冲突
    {
        let x = 5;
        r = &x;//给它一个值之前使用这个变量,会出现一个编译时错误,这就说明了 Rust 确实不允许空值.
    }
    println!("r: {}", r); //尝试使用离开 Scope 的值的 References 
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut list = List::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);
        list.push_back(4);

        assert_eq!(list.pop_back(), Some(4));
        assert_eq!(list.pop_front(), Some(1));
    }
}
