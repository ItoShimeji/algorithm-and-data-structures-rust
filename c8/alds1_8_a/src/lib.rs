struct Node {
    key: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    fn new(key: i32) -> Node {
        Node {
            key,
            left: None,
            right: None,
        }
    }
}

fn search_pre(node: &Option<Box<Node>>, trace: &mut Vec<i32>) {
    if let Some(current) = node {
        trace.push(current.key);
        search_pre(&current.left, trace);
        search_pre(&current.right, trace);
    }
}

fn search_in(node: &Option<Box<Node>>, trace: &mut Vec<i32>) {
    if let Some(current) = node {
        search_in(&current.left, trace);
        trace.push(current.key);
        search_in(&current.right, trace);
    }
}

fn insert(node: &mut Option<Box<Node>>, key: i32) {
    match node {
        Some(current) => {
            if key <= current.key {
                insert(&mut current.left, key);
            } else {
                insert(&mut current.right, key);
            }
        }
        None => *node = Some(Box::new(Node::new(key))),
    }
    // if key <= self.key {
    //     match &mut self.left {
    //         Some(left) => left.as_mut().insert(key),
    //         None => self.left = Some(Box::new(Node::new(key))),
    //     }
    // } else {
    //     match &mut self.right {
    //         Some(right) => right.as_mut().insert(key),
    //         None => self.right = Some(Box::new(Node::new(key))),
    //     }
    // }
}

pub struct BinaryTree {
    root: Option<Box<Node>>,
}

impl BinaryTree {
    pub fn new() -> Self {
        BinaryTree { root: None }
    }

    pub fn insert(&mut self, key: i32) {
        insert(&mut self.root, key);
    }

    pub fn search_pre(&self) -> Vec<i32> {
        let mut trace = Vec::new();
        search_pre(&self.root, &mut trace);
        trace
    }

    pub fn search_in(&self) -> Vec<i32> {
        let mut trace = Vec::new();
        search_in(&self.root, &mut trace);
        trace
    }
}
