// ALDS1_8_C: Binary Search Tree III
// https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ALDS1_8_C
use std::io::{self, Read};

use alds1_8_c::BinaryTree;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Operation {
    Insert(i32),
    Find(i32),
    Delete(i32),
    Print,
}

#[allow(dead_code)]
#[derive(Debug, PartialEq, Eq)]
enum OperationResult {
    Found(bool),
    Walk(WalkResult),
}

#[allow(dead_code)]
#[derive(Debug, PartialEq, Eq)]
struct WalkResult {
    inorder: Vec<i32>,
    preorder: Vec<i32>,
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
fn solve(operations: &[Operation]) -> Vec<OperationResult> {
    let mut tree = BinaryTree::new();
    let mut results: Vec<OperationResult> = Vec::new();

    for operation in operations {
        match operation {
            Operation::Insert(key) => {
                tree.insert(*key);
            }
            Operation::Find(key) => results.push(OperationResult::Found(tree.find(*key))),
            Operation::Delete(key) => {
                tree.delete(*key);
            }
            Operation::Print => {
                results.push(OperationResult::Walk(WalkResult {
                    inorder: tree.search_in(),
                    preorder: tree.search_pre(),
                }));
            }
        };
    }

    results
}

fn run(input: &str) -> String {
    let mut sc = Scanner::new(input);
    let n: usize = sc.next();

    let operations: Vec<Operation> = (0..n)
        .map(|_| {
            let operation: String = sc.next();

            match operation.as_str() {
                "insert" => Operation::Insert(sc.next()),
                "find" => Operation::Find(sc.next()),
                "delete" => Operation::Delete(sc.next()),
                "print" => Operation::Print,
                _ => {
                    panic!();
                }
            }
        })
        .collect();

    let result = solve(&operations);
    let mut output = String::new();

    for operation_result in result {
        match operation_result {
            OperationResult::Found(is_found) => {
                let message = if is_found { "yes" } else { "no" };
                output.push_str(&format!("{}\n", message));
            }
            OperationResult::Walk(traces) => {
                // 空の tree の場合は何も印刷しない
                if traces.inorder.is_empty() {
                    continue;
                }

                let str_inorder = traces
                    .inorder
                    .iter()
                    .map(|n| n.to_string())
                    .collect::<Vec<String>>()
                    .join(" ");

                let str_preorder = traces
                    .preorder
                    .iter()
                    .map(|n| n.to_string())
                    .collect::<Vec<String>>()
                    .join(" ");

                output.push_str(&format!(" {}\n", str_inorder));
                output.push_str(&format!(" {}\n", str_preorder));
            }
        }
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
18
insert 8
insert 2
insert 3
insert 7
insert 22
insert 1
find 1
find 2
find 3
find 4
find 5
find 6
find 7
find 8
print
delete 3
delete 7
print
";

        let output = "\
yes
yes
yes
no
no
no
yes
yes
 1 2 3 7 8 22
 8 2 1 3 7 22
 1 2 8 22
 8 2 1 22
";

        assert_eq!(run(input), output);
    }
}
