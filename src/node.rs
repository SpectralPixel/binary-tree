#[derive(Debug, PartialEq)]
pub struct Node<T> {
    location: i32,
    data: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    pub fn new(location: i32, data: T) -> Self {
        Self {
            location,
            data,
            left: None,
            right: None,
        }
    }

    pub fn insert(&mut self, node: Node<T>) {
        use std::cmp::Ordering::*;
        match node.location.cmp(&self.location) {
            Equal => *self = node,
            Less => match &mut self.left {
                Some(n) => { n.insert(node); },
                None => { self.left = Some(Box::new(node)); }
            },
            Greater => match &mut self.right {
                Some(n) => { n.insert(node); }
                None => { self.right = Some(Box::new(node)); }
            },
        }
    }

    pub fn contains(&self, data: i32) -> bool {
        use std::cmp::Ordering::*;
        match self.location.cmp(&data) {
            Equal => true,
            Less => match &self.right {
                Some(n) => n.contains(data),
                None => false,
            },
            Greater => match &self.left {
                Some(n) => n.contains(data),
                None => false,
            },
        }
    }

    pub fn get(&self, location: i32) -> Option<&Node<T>> {
        use std::cmp::Ordering::*;
        match self.location.cmp(&location) {
            Equal => Some(self),
            Less => match &self.right {
                Some(n) => n.get(location),
                None => None,
            },
            Greater => match &self.left {
                Some(n) => n.get(location),
                None => None,
            },
        }
    }
}
