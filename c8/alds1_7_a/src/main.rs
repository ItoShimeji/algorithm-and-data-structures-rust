// ALDS1_7_A: Rooted Trees
// https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ALDS1_7_A
use std::io::{self, Read};

#[allow(dead_code)]
#[derive(Debug, PartialEq, Eq)]
pub struct NodeInfo {
    pub parent: isize,
    pub depth: usize,
    pub children: Vec<usize>,
}

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

struct Node {
    parent: Option<usize>,
    left: Option<usize>,
    right: Option<usize>,
}

#[allow(dead_code)]
// id, cs
fn solve(children: &[(usize, Vec<usize>)]) -> Vec<NodeInfo> {
    let mut values: Vec<Node> = (0..children.len())
        .map(|_| Node {
            parent: None,
            left: None,
            right: None,
        })
        .collect();

    for (id, cs) in children {
        values[*id].left = cs.first().copied();

        for (index, child_id) in cs.iter().enumerate() {
            // 自分の親を登録
            let child_node = &mut values[*child_id];
            child_node.parent = Some(*id);

            // 左の兄弟の右兄弟として自分を登録
            if index > 0 {
                let left_id = cs[index - 1];
                let left_node = &mut values[left_id];
                left_node.right = Some(*child_id);
            }
        }
    }

    let root = values
        .iter()
        .position(|node| node.parent.is_none())
        .unwrap();

    let mut depth_list: Vec<usize> = vec![0; children.len()];
    set_depth(&values, &mut depth_list, root, 0);

    let mut children_list: Vec<Vec<usize>> = vec![Vec::new(); children.len()];
    set_children(&values, &mut children_list);

    let node_info: Vec<NodeInfo> = (0..children.len())
        .map(|i| NodeInfo {
            parent: values[i].parent.map_or(-1, |p| p as isize),
            depth: depth_list[i],
            children: children_list[i].clone(),
        })
        .collect();

    node_info
}

fn set_depth(values: &Vec<Node>, depth_list: &mut Vec<usize>, id: usize, depth: usize) {
    depth_list[id] = depth;
    if let Some(left) = values[id].left {
        set_depth(values, depth_list, left, depth + 1);
    }
    if let Some(right) = values[id].right {
        set_depth(values, depth_list, right, depth);
    }
}

fn set_children(values: &Vec<Node>, children_list: &mut Vec<Vec<usize>>) {
    for (id, node) in values.iter().enumerate() {
        let mut children: Vec<usize> = Vec::new();

        let mut child = node.left;
        while child.is_some() {
            if let Some(child_id) = child {
                children.push(child_id);
                let child_node = &values[child_id];

                child = child_node.right;
            }
        }

        children_list[id] = children;
    }
}

fn run(input: &str) -> String {
    let mut sc = Scanner::new(input);
    let n = sc.next();

    let nodes: Vec<(usize, Vec<usize>)> = (0..n)
        .map(|_| {
            let id: usize = sc.next();
            let k = sc.next();
            let cs: Vec<usize> = (0..k).map(|_| sc.next()).collect();

            (id, cs)
        })
        .collect();

    let result = solve(&nodes);
    let mut output = String::new();

    for (i, node) in result.iter().enumerate() {
        let node_type = if node.parent == -1 {
            "root"
        } else if node.children.is_empty() {
            "leaf"
        } else {
            "internal node"
        };

        output.push_str(&format!(
            "node {}: parent = {}, depth = {}, {}, {:?}\n",
            i, node.parent, node.depth, node_type, node.children
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
13
0 3 1 4 10
1 2 2 3
2 0
3 0
4 3 5 6 7
5 0
6 0
7 2 8 9
8 0
9 0
10 2 11 12
11 0
12 0
";

        let output = "\
node 0: parent = -1, depth = 0, root, [1, 4, 10]
node 1: parent = 0, depth = 1, internal node, [2, 3]
node 2: parent = 1, depth = 2, leaf, []
node 3: parent = 1, depth = 2, leaf, []
node 4: parent = 0, depth = 1, internal node, [5, 6, 7]
node 5: parent = 4, depth = 2, leaf, []
node 6: parent = 4, depth = 2, leaf, []
node 7: parent = 4, depth = 2, internal node, [8, 9]
node 8: parent = 7, depth = 3, leaf, []
node 9: parent = 7, depth = 3, leaf, []
node 10: parent = 0, depth = 1, internal node, [11, 12]
node 11: parent = 10, depth = 2, leaf, []
node 12: parent = 10, depth = 2, leaf, []
";

        assert_eq!(run(input), output);
    }

    #[test]
    fn sample_2() {
        let input = "\
4
1 3 3 2 0
0 0
3 0
2 0
";

        let output = "\
node 0: parent = 1, depth = 1, leaf, []
node 1: parent = -1, depth = 0, root, [3, 2, 0]
node 2: parent = 1, depth = 1, leaf, []
node 3: parent = 1, depth = 1, leaf, []
";

        assert_eq!(run(input), output);
    }
}
