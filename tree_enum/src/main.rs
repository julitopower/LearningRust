use std::cmp::PartialOrd;
use std::ops::Deref;

/// Generic binary tree defined using an algebraic data type
#[derive(Debug)]
enum Tree<T> {
    Empty,
    Node {
        value: T,
        left: Box<Tree<T>>,
        right: Box<Tree<T>>,
    },
}

impl<T: PartialOrd> Tree<T> {
    /// Create a Tree::Node with the value given
    fn new_node(value: T) -> Tree<T> {
        Tree::Node {
            value,
            left: Box::new(Tree::Empty),
            right: Box::new(Tree::Empty),
        }
    }

    /// Insert a new value in the tree.
    ///
    /// Smaller values go down the left tree. Bigger values go down
    /// the right tree. If the value is already in the tree nothing
    /// happens
    ///
    /// Returns an error if the tree is Empty
    fn insert(self: &mut Tree<T>, val: T) -> Result<(), String> {
        match self {
            Tree::Empty => {
                return Err("Cannot insert value in empty tree".to_string());
            }
            Tree::Node {
                ref value,
                ref mut left,
                ref mut right,
            } => {
                if val < *value {
                    match (*left).deref() {
                        Tree::Empty => *left = Box::new(Tree::new_node(val)),
                        _ => {
                            left.insert(val).unwrap();
                        }                        
                    }
                } else if val > *value {
                    match (*right).deref() {
                        Tree::Empty => *right = Box::new(Tree::new_node(val)),
                        _ => {
                            right.insert(val).unwrap();
                        }                        
                    }
                }
            }
        }
        Ok(())
    }
}

fn main() {
    let mut t = Tree::new_node(2);

    for &x in &[1, 25, -3, 12, 35] {
        t.insert(x).unwrap();
    }
    println!("Hello, world! {:?}", t);
}
