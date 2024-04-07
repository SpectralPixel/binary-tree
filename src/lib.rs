use std::{error::Error, fmt::{self, Debug}};

#[derive(Debug, Clone)]
pub struct DeepestNodeError;
impl Error for DeepestNodeError {}
impl fmt::Display for DeepestNodeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Trying to interact with a node that is smaller than the resoltion")
    }
}

#[derive(Debug, Clone)]
pub struct EmptyTreeError;
impl Error for EmptyTreeError {}
impl fmt::Display for EmptyTreeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Trying to get data from an empty tree.")
    }
}

const SEGMENTS: usize = 2;

#[derive(Debug, PartialEq)]
pub enum Node<T> {
    Data(T),
    Children([Box<Node<T>>; SEGMENTS])
}

#[derive(Debug, PartialEq)]
pub struct Tree<T> {
    size: usize,
    data: Option<Node<T>>
}

impl<T: Copy> Tree<T> {
    pub fn new(depth: usize) -> Self {
        let size = depth.pow(2);
        Tree {
            size,
            data: None,
        }
    }

    pub fn get(&self, pos: usize) -> Result<T, Box<dyn Error>> {
        match &self.data {
            Some(node) => {
                use Node::*;
                match node {
                    Data(rc) => Ok(rc.to_owned()),
                    Children(c) => {
                        let (pos, index) = adjust_local_position(pos, self.size);
                        Ok(c.get(index).unwrap().get_recursive(pos, self.size / 2))
                    }
                }
            },
            None => Err(Box::new(EmptyTreeError)),
        }
    }

    // pub fn insert(&mut )
}

impl<T: Copy> Node<T> {
    pub fn get_recursive(&self, pos: usize, size: usize) -> T {
        use Node::*;
        match self {
            Data(d) => d.to_owned(),
            Children(c) => {
                let (pos, index) = adjust_local_position(pos, size);
                c.get(index).unwrap().get_recursive(pos, size / 2)
            }
        }
    }
}

fn adjust_local_position(mut pos: usize, size: usize) -> (usize, usize) {
    let index = pos * SEGMENTS / size;
    pos -= index * (size / SEGMENTS);
    (pos, index)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_tree() {
        let result: Tree<u8> = Tree::new(2);
        let expected: Tree<u8> = Tree { size: 4, data: None };
        assert_eq!(result, expected);
    }

    #[test]
    fn adjust_pos_1() {
        let (adjusted_pos, local_pos) = adjust_local_position(3, 4);
        let expected_adj_pos = 1;
        let expected_loc_pos = 1;
        assert_eq!(adjusted_pos, expected_adj_pos);
        assert_eq!(local_pos, expected_loc_pos);
    }

    #[test]
    fn adjust_pos_2() {
        let (adjusted_pos, local_pos) = adjust_local_position(6, 8);
        let expected_adj_pos = 2;
        let expected_loc_pos = 1;
        assert_eq!(adjusted_pos, expected_adj_pos);
        assert_eq!(local_pos, expected_loc_pos);
    }

    #[test]
    fn adjust_pos_3() {
        let (adjusted_pos, local_pos) = adjust_local_position(7, 8);
        let expected_adj_pos = 3;
        let expected_loc_pos = 1;
        assert_eq!(adjusted_pos, expected_adj_pos);
        assert_eq!(local_pos, expected_loc_pos);
    }

    #[test]
    fn fragile_getter_test() {
        // It's fragile because any little change to the stucture of the data type would BREAK EVERYTHING AND TAKE AGES TO REFACTOR
        let tree = Tree {
            size: 8,
            data: Some(Node::Children([
                Box::new(Node::Children([
                    Box::new(Node::Data(34)),
                    Box::new(Node::Children([
                        Box::new(Node::Data(22)),
                        Box::new(Node::Data(61)),
                    ])),
                ])),
                Box::new(Node::Data(75)),
            ])),
        };
        assert_eq!(tree.get(0).unwrap(), 34);
        assert_eq!(tree.get(1).unwrap(), 34);
        assert_eq!(tree.get(2).unwrap(), 22);
        assert_eq!(tree.get(3).unwrap(), 61);
        assert_eq!(tree.get(4).unwrap(), 75);
        assert_eq!(tree.get(5).unwrap(), 75);
        assert_eq!(tree.get(6).unwrap(), 75);
        assert_eq!(tree.get(7).unwrap(), 75);
    }
}
