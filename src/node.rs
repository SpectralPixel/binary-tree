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
}
