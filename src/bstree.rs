use std::cmp::{Ord, Ordering};
use std::path::PathBuf;

#[derive(Debug)]
pub enum BST<T: Ord> {
    Leaf {
        name: T,
        value: PathBuf,
        left: Box<BST<T>>,
        right: Box<BST<T>>,
    },
    Empty,
}

impl<T: Ord> BST<T> {
    pub fn new() -> Self {
        BST::Empty
    }

    pub fn create(name: T, value: PathBuf) -> Self {
        BST::Leaf {
            name,
            value,
            left: Box::new(BST::Empty),
            right: Box::new(BST::Empty),
        }
    }

    pub fn insert(&mut self, new_name: T, new_value: PathBuf) {
        match self {
            BST::Leaf {
                ref name,
                value: _,
                ref mut left,
                ref mut right,
            } => match new_name.cmp(name) {
                Ordering::Less => left.insert(new_name, new_value),
                Ordering::Greater => right.insert(new_name, new_value),
                _ => return,
            },
            BST::Empty => {
                *self = BST::create(new_name, new_value);
            }
        }
    }

    //   pub fn is_empty(&self) -> bool {
    //       match self {
    //          BST::Empty => true,
    //          BST::Leaf { .. } => false,
    //      }
    //  }

    pub fn find(&self, find_name: T) -> Option<PathBuf> {
        match self {
            BST::Leaf {
                ref name,
                ref value,
                ref left,
                ref right,
            } => match find_name.cmp(name) {
                Ordering::Less => left.find(find_name),
                Ordering::Greater => right.find(find_name),
                Ordering::Equal => Some(value.to_owned()),
            },
            BST::Empty => None,
        }
    }
}

impl<T: Ord> Default for BST<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn create() {
        //let mut t1 = BST::new();
        //t1.insert(1);
        //t1.insert(2);
        //println!("{:?}", t1)
    }

    #[test]
    fn find() {
        //assert_eq!(1, 0);
        // let mut t1 = BST::new();
        //t1.insert(3);
        //t1.insert(1);
        //t1.insert(2);
        //assert_eq!(true, t1.find(2));
        //assert_eq!(false, t1.find(5));
    }
}
