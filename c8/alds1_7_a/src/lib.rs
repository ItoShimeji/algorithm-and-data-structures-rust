#[allow(dead_code)]
#[derive(Debug, PartialEq, Eq)]
pub struct NodeInfo {
    pub id: usize,
    pub parent: isize,
    pub depth: usize,
    pub children: Vec<usize>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Node {
    id: usize,
    children: Vec<Node>,
}

impl Node {
    pub fn new(id: usize) -> Self {
        Node {
            id,
            children: Vec::new(),
        }
    }

    fn search_mut(&mut self, id: usize) -> Option<&mut Self> {
        if self.id == id {
            return Some(self);
        };

        for child in &mut self.children {
            if let Some(node) = child.search_mut(id) {
                return Some(node);
            }
        }

        None
    }

    fn search(&self, info_list: &mut Vec<NodeInfo>, parent: isize, depth: usize) {
        let node_info = NodeInfo {
            id: self.id,
            parent,
            depth,
            children: self.children.iter().map(|c| c.id).collect(),
        };

        info_list.push(node_info);

        for child in &self.children {
            child.search(info_list, self.id as isize, depth + 1);
        }
    }
}

pub struct Tree {
    root: Option<Node>,
}

impl Tree {
    pub fn new() -> Self {
        Tree { root: None }
    }

    pub fn insert(&mut self, id: usize, children: Vec<usize>) {
        let Some(mut root) = self.root.take() else {
            let mut root = Node::new(id);
            root.children = children.into_iter().map(Node::new).collect();
            self.root = Some(root);
            return;
        };

        match root.search_mut(id) {
            Some(node) => {
                node.children = children.into_iter().map(Node::new).collect();
                self.root = Some(root);
            }
            None => {
                let mut new_root = Node::new(id);
                new_root.children = vec![root];
                self.root = Some(new_root);
            }
        }
    }

    // fn search_mut(&mut self, id: usize) -> Option<&mut Node> {
    //     match &mut self.root {
    //         Some(node) => node.search_mut(id),
    //         None => None,
    //     }
    // }

    pub fn get_node_info(&self) -> Vec<NodeInfo> {
        let mut info_list: Vec<NodeInfo> = Vec::new();

        if let Some(root) = &self.root {
            root.search(&mut info_list, -1, 0);
        }

        info_list.sort_by_key(|node| node.id);
        info_list
    }
}
