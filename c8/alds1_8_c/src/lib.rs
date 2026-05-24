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

    fn is_no_child(&self) -> bool {
        self.left.is_none() && self.right.is_none()
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

// inorder において、次の node である次節点の key を返す
fn take_min(node: &Node) -> i32 {
    match &node.left {
        Some(left) => take_min(left),
        None => node.key,
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
}

fn find(node: &Option<Box<Node>>, key: i32) -> bool {
    match node {
        Some(current) => {
            if key < current.key {
                find(&current.left, key)
            } else if key == current.key {
                true
            } else {
                find(&current.right, key)
            }
        }
        None => false,
    }
}

fn delete(node: &mut Option<Box<Node>>, key: i32) {
    if let Some(current) = node {
        if key < current.key {
            delete(&mut current.left, key);
            return;
        } else if current.key < key {
            delete(&mut current.right, key);
            return;
        }

        // key == current.key の時

        // 子供が存在しない時
        if current.is_no_child() {
            *node = None;
            return;
        }

        // 左だけ存在
        if current.left.is_some() && current.right.is_none() {
            // node は current への参照であり、これ自体を変更することで、
            // 親から操作しなくても自分を付け替えることができる。
            *node = current.left.take();
            return;
        }

        // 右だけ存在
        if current.left.is_none() && current.right.is_some() {
            *node = current.right.take();
            return;
        }

        if let Some(right) = &current.right {
            // 右側木の最小値と交換
            let next_key = take_min(right);
            current.key = next_key;

            // 交換後の node を削除
            delete(&mut current.right, next_key);
        }
    }
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

    pub fn find(&self, key: i32) -> bool {
        find(&self.root, key)
    }

    pub fn delete(&mut self, key: i32) {
        delete(&mut self.root, key);
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

#[cfg(test)]
mod tests {
    use super::BinaryTree;

    fn tree_from(keys: &[i32]) -> BinaryTree {
        let mut tree = BinaryTree::new();

        for key in keys {
            tree.insert(*key);
        }

        tree
    }

    #[test]
    fn delete_root_with_two_children_successor_is_right_child() {
        let mut tree = tree_from(&[5, 3, 7, 8]);

        tree.delete(5);

        assert_eq!(tree.search_in(), vec![3, 7, 8]);
        assert_eq!(tree.search_pre(), vec![7, 3, 8]);
        assert!(!tree.find(5));
    }

    #[test]
    fn delete_root_with_two_children_successor_has_right_child() {
        let mut tree = tree_from(&[5, 3, 9, 7, 8]);

        tree.delete(5);

        assert_eq!(tree.search_in(), vec![3, 7, 8, 9]);
        assert_eq!(tree.search_pre(), vec![7, 3, 9, 8]);
        assert!(!tree.find(5));
    }

    #[test]
    fn delete_internal_node_with_two_children() {
        let mut tree = tree_from(&[10, 5, 15, 3, 7, 6, 8]);

        tree.delete(5);

        assert_eq!(tree.search_in(), vec![3, 6, 7, 8, 10, 15]);
        assert_eq!(tree.search_pre(), vec![10, 6, 3, 7, 8, 15]);
        assert!(!tree.find(5));
    }
}
