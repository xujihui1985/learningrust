#[derive(Debug)]
enum StatusCode {
    OK,
}

struct Point {
    x: i32,
    y: i32,
    z: i32,
}

fn http_status_from_u32(n: u32) -> Option<StatusCode> {
    match n {
        200 => Some(StatusCode::OK),
        _ => None
    }
}

struct TreeNode<T> {
    element: T,
    left: BinaryTree<T>,
    right: BinaryTree<T>,
}

enum BinaryTree<T> {
    Empty,
    NonEmpty(Box<TreeNode<T>>),
}

impl<T: Ord> BinaryTree<T> {
    fn add(&mut self, value: T) {
        match *self {
            BinaryTree::Empty => {
                *self = BinaryTree::NonEmpty(Box::new(TreeNode {
                    element: value,
                    left: BinaryTree::Empty,
                    right: BinaryTree::Empty,
                }))
            },
            BinaryTree::NonEmpty(ref mut node) => {
                if value <= node.element {
                    node.left.add(value);
                } else {
                    node.right.add(value);
                }
            }
        }
    }
}

fn main() {

    let mut tree = BinaryTree::Empty;
    tree.add("root");
    tree.add("hello");

    match http_status_from_u32(200) {
        Some(ref status) => println!("status is {:?}", status),
        _ => println!("invalid status")
    }

    let p = &Point {
        x: 1,
        y: 2,
        z: 3,
    };

    match p {
        &Point { x, y, z } => {
            println!("x {} y {} z {}", x, y, z)
        }
    }

    match p {
        &Point { ref x, .. } => {
            println!("x {}", x)
        }
    }
}
