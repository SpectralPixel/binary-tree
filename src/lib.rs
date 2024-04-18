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
}
