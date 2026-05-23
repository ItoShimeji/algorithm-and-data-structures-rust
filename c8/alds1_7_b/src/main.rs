// ALDS1_7_B: Binary Tree
// https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ALDS1_7_B
use std::{
    cmp,
    io::{self, Read},
};

struct Scanner {
    input: Vec<String>,
    index: usize,
}

impl Scanner {
    fn new(input: &str) -> Self {
        let input = input.split_whitespace().map(String::from).collect();
        Scanner { input, index: 0 }
    }

    fn next<T: std::str::FromStr>(&mut self) -> T {
        let value = self.input[self.index].parse::<T>().ok().unwrap();
        self.index += 1;
        value
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NodeInput {
    pub id: usize,
    pub left: Option<usize>,
    pub right: Option<usize>,
}

#[allow(dead_code)]
#[derive(Debug, PartialEq, Eq)]
pub struct NodeInfo {
    pub parent: Option<usize>,
    pub sibling: Option<usize>,
    pub degree: usize,
    pub depth: usize,
    pub height: usize,
}

struct Node {
    parent: Option<usize>,
    left: Option<usize>,
    right: Option<usize>,
}

#[allow(dead_code)]
fn solve(nodes: &[NodeInput]) -> Vec<NodeInfo> {
    let (tree, root_id) = create_tree(nodes);

    let mut node_info: Vec<NodeInfo> = (0..nodes.len())
        .map(|_| NodeInfo {
            parent: None,
            sibling: None,
            degree: 0,
            depth: 0,
            height: 0,
        })
        .collect();

    set_info(&tree, &mut node_info, root_id, None, None, 0);

    node_info
}

// height を返す
fn set_info(
    tree: &Vec<Node>,
    node_info: &mut Vec<NodeInfo>,
    id: usize,
    parent_id: Option<usize>,
    sibling: Option<usize>,
    depth: usize,
) -> usize {
    let mut degree = 0;
    let h_left = if let Some(left) = tree[id].left {
        degree += 1;
        set_info(tree, node_info, left, Some(id), tree[id].right, depth + 1) + 1
    } else {
        0
    };

    let h_right = if let Some(right) = tree[id].right {
        degree += 1;
        set_info(tree, node_info, right, Some(id), tree[id].left, depth + 1) + 1
    } else {
        0
    };

    let height = cmp::max(h_left, h_right);

    // 値をセット
    node_info[id] = NodeInfo {
        parent: parent_id,
        sibling,
        degree,
        depth,
        height,
    };

    height
}

// tree と root index を返す
fn create_tree(nodes: &[NodeInput]) -> (Vec<Node>, usize) {
    let mut tree: Vec<Node> = (0..nodes.len())
        .map(|i| {
            // 0..n の id が順不同で与えられることを想定
            // tree のインデックス i には id == i の node を登録
            let node = nodes.iter().find(|&node| node.id == i).unwrap();

            Node {
                parent: None,
                left: node.left,
                right: node.right,
            }
        })
        .collect();

    let mut has_parent: Vec<bool> = vec![false; tree.len()];
    for node in &tree {
        if let Some(left) = node.left {
            has_parent[left] = true;
        }
        if let Some(right) = node.right {
            has_parent[right] = true;
        }
    }
    // 親を持たないインデックスの node が一つだけあるはず
    let root_index = has_parent.iter().position(|n| !n).unwrap();
    set_parent(&mut tree, root_index, None);

    (tree, root_index)
}

// id: parent を設定する id
// parent id: id の親
fn set_parent(tree: &mut Vec<Node>, id: usize, parent_id: Option<usize>) {
    tree[id].parent = parent_id;

    if let Some(left) = tree[id].left {
        set_parent(tree, left, Some(id));
    }
    if let Some(right) = tree[id].right {
        set_parent(tree, right, Some(id));
    }
}

fn run(input: &str) -> String {
    let mut sc = Scanner::new(input);
    let n: usize = sc.next();

    let node_input: Vec<NodeInput> = (0..n)
        .map(|_| {
            let id = sc.next();

            let left: isize = sc.next();
            let left = if left != -1 {
                Some(left as usize)
            } else {
                None
            };

            let right: isize = sc.next();
            let right = if right != -1 {
                Some(right as usize)
            } else {
                None
            };

            NodeInput { id, left, right }
        })
        .collect();

    let result = solve(&node_input);
    let mut output = String::new();

    for (i, node) in result.iter().enumerate() {
        let node_type = if node.parent.is_none() {
            "root"
        } else if node.height > 0 {
            "internal node"
        } else {
            "leaf"
        };

        output.push_str(&format!(
            "node {}: parent = {}, sibling = {}, degree = {}, depth = {}, height = {}, {}\n",
            i,
            node.parent.map_or(-1, |p| { p as isize }),
            node.sibling.map_or(-1, |s| { s as isize }),
            node.degree,
            node.depth,
            node.height,
            node_type
        ));
    }

    output
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    print!("{}", run(&input));
}

#[cfg(test)]
mod tests {
    use super::run;

    #[test]
    fn sample_1() {
        let input = "\
9
0 1 4
1 2 3
2 -1 -1
3 -1 -1
4 5 8
5 6 7
6 -1 -1
7 -1 -1
8 -1 -1
";

        let output = "\
node 0: parent = -1, sibling = -1, degree = 2, depth = 0, height = 3, root
node 1: parent = 0, sibling = 4, degree = 2, depth = 1, height = 1, internal node
node 2: parent = 1, sibling = 3, degree = 0, depth = 2, height = 0, leaf
node 3: parent = 1, sibling = 2, degree = 0, depth = 2, height = 0, leaf
node 4: parent = 0, sibling = 1, degree = 2, depth = 1, height = 2, internal node
node 5: parent = 4, sibling = 8, degree = 2, depth = 2, height = 1, internal node
node 6: parent = 5, sibling = 7, degree = 0, depth = 3, height = 0, leaf
node 7: parent = 5, sibling = 6, degree = 0, depth = 3, height = 0, leaf
node 8: parent = 4, sibling = 5, degree = 0, depth = 2, height = 0, leaf
";

        assert_eq!(run(input), output);
    }
}
