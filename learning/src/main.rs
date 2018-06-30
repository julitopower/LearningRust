/**
 * Represents a node on a tree. A node is a Generic container.
 */
#[derive(Debug)]
struct Node<T: std::cmp::PartialEq> {
    /// The payload of this Node
    value: T,
    /// Optional Box to left child Node
    left: Option<Box<Node<T>>>,
    /// Optional Box to right child Node
    right: Option<Box<Node<T>>>,
}

impl<T: std::cmp::PartialOrd> Node<T> {
    pub fn new(v: T) -> Node<T> {
        Node {
            value: v,
            left: None,
            right: None,
        }
    }

    pub fn insert(self: &mut Node<T>, v: T) {
        if v == self.value {
            return;
        } else if v < self.value {
            match self.left {
                Some(ref mut node) => node.insert(v),
                None => self.left = Some(Box::new(Node::new(v))),
            }
        } else {
            match self.right {
                Some(ref mut node) => node.insert(v),
                None => self.right = Some(Box::new(Node::new(v))),
            }
        }
    }

    pub fn contains(self: &Node<T>, v: T) -> bool {
        if v == self.value {
            return true;
        } else if v < self.value {
            match self.left {
                Some(ref node) => node.contains(v),
                None => false,
            }
        } else {
            match self.right {
                Some(ref node) => node.contains(v),
                None => false,
            }
        }
    }
}

fn main() {
    println!("Hello, world!");
    let x = vec![1, 2, 3];
    for i in x.iter() {
        println!("{}", i);
    }
    println!("{:?}", x);
    let mut tree = Node::new(23_i32);
    tree.insert(12);
    tree.insert(44);

    println!("{:?}", tree);
    assert_eq!(true, tree.contains(12));
    assert_eq!(false, tree.contains(11));
}
