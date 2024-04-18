pub struct Node {
    data: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    pub fn new(data: i32) -> Self {
        Self {
            data,
            left: None,
            right: None,
        }
    }

    pub fn insert(&mut self, node: Node) {
        use std::cmp::Ordering::*;
        match node.data.cmp(&self.data) {
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
}
