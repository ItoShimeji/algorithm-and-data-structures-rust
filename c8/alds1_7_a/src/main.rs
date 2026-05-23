// ALDS1_7_A: Rooted Trees
// https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ALDS1_7_A
use std::io::{self, Read};

use alds1_7_a::{NodeInfo, Tree};

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
// id, cs
fn solve(children: &[(usize, Vec<usize>)]) -> Vec<NodeInfo> {
    let mut tree = Tree::new();
    for (id, children) in children {
        tree.insert(*id, children.clone());
    }

    tree.get_node_info()
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

    for node in result {
        let node_type = if node.parent == -1 {
            "root"
        } else if node.children.is_empty() {
            "leaf"
        } else {
            "internal node"
        };

        output.push_str(&format!(
            "node {}: parent = {}, depth = {}, {}, {:?}\n",
            node.id, node.parent, node.depth, node_type, node.children
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
