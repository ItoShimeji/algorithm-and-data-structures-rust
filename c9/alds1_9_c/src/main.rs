// ALDS1_9_C: Priority Queue
// https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ALDS1_9_C
use std::io::{self, Read};

use alds1_9_c::Heap;

#[derive(Debug, PartialEq, Eq)]
enum Operation {
    Insert(i64),
    Extract,
    End,
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

fn solve(operations: &[Operation]) -> Vec<i64> {
    let mut heap = Heap::new();
    let mut result: Vec<i64> = Vec::new();

    for operation in operations {
        match operation {
            Operation::Insert(key) => heap.insert(*key),
            Operation::Extract => {
                if let Some(max) = heap.extract_max() {
                    result.push(max)
                }
            }
            Operation::End => break,
        }
    }

    result
}

fn run(input: &str) -> String {
    let mut sc = Scanner::new(input);
    let mut operations: Vec<Operation> = Vec::new();

    loop {
        let command: String = sc.next();
        match command.as_str() {
            "insert" => operations.push(Operation::Insert(sc.next())),
            "extract" => operations.push(Operation::Extract),
            "end" => {
                operations.push(Operation::End);
                break;
            }
            _ => panic!(),
        };
    }

    solve(&operations)
        .iter()
        .map(|v| format!("{}\n", v))
        .collect::<String>()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    print!("{}", run(&input));
}

#[cfg(test)]
mod tests {
    use super::{run, solve, Operation};

    #[test]
    fn sample_1() {
        let input = "\
insert 8
insert 2
extract
insert 10
extract
insert 11
extract
extract
end
";

        let output = "\
8
10
11
2
";

        assert_eq!(run(input), output);
    }

    #[test]
    fn sample_1_solve() {
        let operations = vec![
            Operation::Insert(8),
            Operation::Insert(2),
            Operation::Extract,
            Operation::Insert(10),
            Operation::Extract,
            Operation::Insert(11),
            Operation::Extract,
            Operation::Extract,
            Operation::End,
        ];

        assert_eq!(solve(&operations), vec![8, 10, 11, 2]);
    }
}
