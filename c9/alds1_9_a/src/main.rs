// ALDS1_9_A: Complete Binary Tree
// https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ALDS1_9_A
use std::io::{self, Read};

#[allow(dead_code)]
#[derive(Debug, PartialEq, Eq)]
struct NodeProperty {
    id: usize,
    key: i32,
    parent_key: Option<i32>,
    left_key: Option<i32>,
    right_key: Option<i32>,
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

#[allow(dead_code)]
fn solve(keys: &[i32]) -> Vec<NodeProperty> {
    let mut nodes: Vec<NodeProperty> = Vec::new();
    for (i, &key) in keys.iter().enumerate() {
        let id = i + 1;
        // 条件分岐は usize のオーバーフローが起きないために必要
        let parent_key = if id == 1 {
            None
        } else {
            Some(keys[id / 2 - 1])
        };
        let left_key = keys.get(2 * id - 1).copied();
        let right_key = keys.get(2 * id).copied();

        nodes.push(NodeProperty {
            id,
            key,
            parent_key,
            left_key,
            right_key,
        })
    }

    nodes
}

fn run(input: &str) -> String {
    let mut sc = Scanner::new(input);
    let n: usize = sc.next();
    let keys: Vec<i32> = (0..n).map(|_| sc.next()).collect();

    let result = solve(&keys);
    let mut output = String::new();

    for property in result {
        output.push_str(&format!("node {}: key = {},", property.id, property.key));
        if let Some(parent_key) = property.parent_key {
            output.push_str(&format!(" parent key = {},", parent_key));
        }
        if let Some(left_key) = property.left_key {
            output.push_str(&format!(" left key = {},", left_key));
        }
        if let Some(right_key) = property.right_key {
            output.push_str(&format!(" right key = {},", right_key));
        }
        output.push('\n');
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
5
7 8 1 2 3
";

        let output = "\
node 1: key = 7, left key = 8, right key = 1,
node 2: key = 8, parent key = 7, left key = 2, right key = 3,
node 3: key = 1, parent key = 7,
node 4: key = 2, parent key = 8,
node 5: key = 3, parent key = 8,
";

        assert_eq!(run(input), output);
    }
}
