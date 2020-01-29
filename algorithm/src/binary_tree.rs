use std::mem;

#[derive(Clone, Debug)]
pub struct IoTDevice {
    pub id: u64,
    pub address: String,
}

type Tree = Option<Box<Node>>;

struct Node {
    pub dev: IoTDevice,
    left: Tree,
    right: Tree,
}

impl Node {
    pub fn new(dev: IoTDevice) -> Box<Node> {
        Box::new(Node {
            dev,
            left: None,
            right: None,
        })
    }
}

pub struct BinaryTree {
    root: Tree,
    pub length: u64,
}

impl BinaryTree {
    pub fn new() -> BinaryTree {
        BinaryTree {
            root: None,
            length: 0,
        }
    }
    pub fn add(&mut self, device: IoTDevice) {
        self.length += 1;
        let root = mem::replace(&mut self.root, None);
        self.root = self.add_rec(root, device);
    }

    pub fn find(&self, id: u64) -> Option<IoTDevice> {
        self.find_r(&self.root, id)
    }

    pub fn walk(&self, mut callback: impl FnMut(&IoTDevice) -> ()) {
        self.walk_in_order(&self.root, &mut callback);
    }

    fn walk_in_order(&self, node: &Tree, callback: &mut impl FnMut(&IoTDevice) -> ()) {
        if let Some(n) = node {
            self.walk_in_order(&n.left, callback);
            callback(&n.dev);
            self.walk_in_order(&n.right, callback);
        }
    }

    fn find_r(&self, node: &Tree, id: u64) -> Option<IoTDevice> {
        match node {
            Some(n) => {
                if n.dev.id == id {
                    Some(n.dev.clone())
                } else if n.dev.id < id {
                    self.find_r(&n.left, id)
                } else {
                    self.find_r(&n.right, id)
                }
            }
            _ => None,
        }
    }

    fn add_rec(&mut self, node: Tree, device: IoTDevice) -> Tree {
        match node {
            Some(mut n) => {
                if n.dev.id <= device.id {
                    n.left = self.add_rec(n.left, device);
                    Some(n)
                } else {
                    n.right = self.add_rec(n.right, device);
                    Some(n)
                }
            }
            _ => Some(Node::new(device))
        }
    }
}