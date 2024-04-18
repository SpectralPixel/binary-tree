use crate::node::Node;

mod node;

#[derive(Default)]
pub struct Tree {
    root: Option<Node>,
}

impl Tree {
    pub fn insert(&mut self, node: Node) {
        match &mut self.root {
            Some(r) => {
                r.insert(node);
            },
            None => {
                self.root = Some(node);
            }
        }
    }
}
