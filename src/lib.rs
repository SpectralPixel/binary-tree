use crate::node::Node;

mod node;

#[derive(Debug, PartialEq)]
pub struct Tree<T: Default + PartialEq> {
    root: Option<Node<T>>,
    size: usize,
}

impl<T: Default + PartialEq> Default for Tree<T> {
    fn default() -> Self {
        Self {
            root: None,
            size: 8,
        }
    }
}

impl<T: Default + PartialEq> Tree<T> {
    pub fn insert(&mut self, node: Node<T>) {
        match &mut self.root {
            Some(r) => {
                r.insert(node);
            },
            None => {
                self.root = Some(node);
            }
        }
    }

    pub fn contains(&self, location: i32, data: T) -> bool {
        match &self.root {
            Some(r) => {
                match r.get(location) {
                    Some(n) => *n.data() == data,
                    None => false,
                }
            },
            None => {
                false
            }
        }
    }

    pub fn get(&self, location: i32) -> Option<&Node<T>> {
        match &self.root {
            Some(r) => {
                r.get(location)
            },
            None => {
                None
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Tree;
    use crate::node::Node;

    #[test]
    fn default_tree() {
        assert_eq!(
            Tree::default(),
            Tree::<i32> { root: None, size: 8 }
        )
    }

    #[test]
    fn tree_contains() {
        let mut tree = Tree::default();
        tree.insert(Node::new(3, 3));
        tree.insert(Node::new(7, 7));
        tree.insert(Node::new(9, 9));
        tree.insert(Node::new(1, 1));
        tree.insert(Node::new(2, 2));
        tree.insert(Node::new(0, 0));
        assert!(tree.contains(3, 3));
        assert!(tree.contains(7, 7));
        assert!(tree.contains(9, 9));
        assert!(tree.contains(1, 1));
        assert!(tree.contains(2, 2));
        assert!(tree.contains(0, 0));
    }
}
