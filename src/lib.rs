use crate::node::Node;

mod node;

#[derive(Default, Debug, PartialEq)]
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

#[cfg(test)]
mod tests {
    use crate::Tree;
    use crate::node::Node;

    #[test]
    fn default_tree() {
        assert_eq!(
            Tree::default(),
            Tree { root: None }
        )
    }

    #[test]
    fn print_empty_tree() {
        assert_eq!(
            format!("{:#?}", Tree::default()),
           "\
Tree {
    root: None,
}"
        )
    }

    #[test]
    fn insert_root() {
        let mut tree = Tree::default();
        tree.insert(Node::new(3));
        assert_eq!(
            format!("{:#?}", tree),
            "\
Tree {
    root: Some(
        Node {
            data: 3,
            left: None,
            right: None,
        },
    ),
}"
        )
    }

    #[test]
    fn insert_many() {
        let mut tree = Tree::default();
        tree.insert(Node::new(3));
        tree.insert(Node::new(7));
        tree.insert(Node::new(9));
        tree.insert(Node::new(1));
        tree.insert(Node::new(2));
        tree.insert(Node::new(0));
        assert_eq!(
            format!("{:#?}", tree),
            "\
Tree {
    root: Some(
        Node {
            data: 3,
            left: Some(
                Node {
                    data: 1,
                    left: Some(
                        Node {
                            data: 0,
                            left: None,
                            right: None,
                        },
                    ),
                    right: Some(
                        Node {
                            data: 2,
                            left: None,
                            right: None,
                        },
                    ),
                },
            ),
            right: Some(
                Node {
                    data: 7,
                    left: None,
                    right: Some(
                        Node {
                            data: 9,
                            left: None,
                            right: None,
                        },
                    ),
                },
            ),
        },
    ),
}"
        )
    }
}
