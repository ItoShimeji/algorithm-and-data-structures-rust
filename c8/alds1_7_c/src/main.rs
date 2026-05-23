// ALDS1_7_C: Tree Walk
// https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ALDS1_7_C
use std::io::{self, Read};

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NodeInput {
    pub id: usize,
    pub left: Option<usize>,
    pub right: Option<usize>,
}

#[allow(dead_code)]
#[derive(Debug, PartialEq, Eq)]
pub struct WalkResult {
    pub preorder: Vec<usize>,
    pub inorder: Vec<usize>,
    pub postorder: Vec<usize>,
}

#[allow(dead_code)]
struct Scanner {
    input: Vec<String>,
    index: usize,
}

#[allow(dead_code)]
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
    left: Option<usize>,
    right: Option<usize>,
}

fn create_tree(nodes: &[NodeInput]) -> (Vec<Node>, usize) {
    let tree: Vec<Node> = (0..nodes.len())
        .map(|i| {
            // 0..n の id が順不同で与えられることを想定すると、以下のようにするか、sortしてからか
            // tree のインデックス i には id == i の node を登録
            // let node = nodes.iter().find(|&node| node.id == i).unwrap();

            Node {
                left: nodes[i].left,
                right: nodes[i].right,
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

    (tree, root_index)
}

fn search_pre(tree: &Vec<Node>, trace: &mut Vec<usize>, id: usize) {
    trace.push(id);
    if let Some(left) = tree[id].left {
        search_pre(tree, trace, left);
    }
    if let Some(right) = tree[id].right {
        search_pre(tree, trace, right);
    }
}

fn search_in(tree: &Vec<Node>, trace: &mut Vec<usize>, id: usize) {
    if let Some(left) = tree[id].left {
        search_in(tree, trace, left);
    }
    trace.push(id);
    if let Some(right) = tree[id].right {
        search_in(tree, trace, right);
    }
}

fn search_post(tree: &Vec<Node>, trace: &mut Vec<usize>, id: usize) {
    if let Some(left) = tree[id].left {
        search_post(tree, trace, left);
    }
    if let Some(right) = tree[id].right {
        search_post(tree, trace, right);
    }
    trace.push(id);
}

#[allow(dead_code)]
fn solve(nodes: &[NodeInput]) -> WalkResult {
    let (tree, root_id) = create_tree(nodes);

    let mut trace_pre = Vec::new();
    let mut trace_in = Vec::new();
    let mut trace_post = Vec::new();

    search_pre(&tree, &mut trace_pre, root_id);
    search_in(&tree, &mut trace_in, root_id);
    search_post(&tree, &mut trace_post, root_id);

    WalkResult {
        preorder: trace_pre,
        inorder: trace_in,
        postorder: trace_post,
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

    output.push_str(&format!("Preorder\n"));
    let trace = join_trace(result.preorder);
    output.push_str(&format!(" {}\n", trace));

    output.push_str(&format!("Inorder\n"));
    let trace = join_trace(result.inorder);
    output.push_str(&format!(" {}\n", trace));

    output.push_str(&format!("Postorder\n"));
    let trace = join_trace(result.postorder);
    output.push_str(&format!(" {}\n", trace));

    output
}

fn join_trace(trace: Vec<usize>) -> String {
    trace
        .iter()
        .map(|e| e.to_string())
        .collect::<Vec<String>>()
        .join(" ")
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
Preorder
 0 1 2 3 4 5 6 7 8
Inorder
 2 1 3 0 6 5 7 4 8
Postorder
 2 3 1 6 7 5 8 4 0
";

        assert_eq!(run(input), output);
    }
}
