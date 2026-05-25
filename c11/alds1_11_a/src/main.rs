// ALDS1_11_A: Graph
// https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ALDS1_11_A
use std::io::{self, Read};

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
fn solve(n: usize, adjacency_lists: &[Vec<usize>]) -> Vec<Vec<usize>> {
    let mut matrix = vec![vec![0usize; n]; n];

    for (i, adjacency_list) in adjacency_lists.iter().enumerate() {
        for &a in adjacency_list {
            // a は 頂点番号で、インデックスより 1 大きい
            matrix[i][a - 1] = 1;
        }
    }

    matrix
}

fn run(input: &str) -> String {
    let mut sc = Scanner::new(input);
    let n: usize = sc.next();
    let adjacency_lists: Vec<Vec<usize>> = (0..n)
        .map(|_| {
            sc.next::<usize>();
            let k: usize = sc.next();

            (0..k).map(|_| sc.next()).collect::<Vec<usize>>()
        })
        .collect();

    solve(n, &adjacency_lists)
        .iter()
        .map(|adjacency_list| {
            adjacency_list
                .iter()
                .map(|a| a.to_string())
                .collect::<Vec<String>>()
                .join(" ")
        })
        .map(|s| format!("{}\n", s))
        .collect::<String>()
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
4
1 2 2 4
2 1 4
3 0
4 1 3
";

        let output = "\
0 1 0 1
0 0 0 1
0 0 0 0
0 0 1 0
";

        assert_eq!(run(input), output);
    }
}
