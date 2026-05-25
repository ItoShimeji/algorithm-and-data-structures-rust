// ALDS1_11_C: Breadth First Search
// https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ALDS1_11_C
use std::{
    collections::VecDeque,
    io::{self, Read},
};

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

fn to_matrix(n: usize, adjacency_lists: &[Vec<usize>]) -> Vec<Vec<bool>> {
    let mut matrix = vec![vec![false; n]; n];

    for (i, adjacency_list) in adjacency_lists.iter().enumerate() {
        for &a in adjacency_list {
            // a は 頂点番号で、インデックスより 1 大きい
            matrix[i][a - 1] = true;
        }
    }

    matrix
}

#[derive(Clone, Copy)]
enum Color {
    White,
    Gray,
    Black,
}

#[allow(dead_code)]
fn solve(n: usize, adjacency_lists: &[Vec<usize>]) -> Vec<(usize, i32)> {
    // この問題だと、 queue と colors の責務が被っている
    // 最短距離ではなく、閉路の検出などでは活用できるらしい
    let mut colors = vec![Color::White; n];
    let matrix = to_matrix(n, adjacency_lists);
    // vertex id を登録
    let mut queue: VecDeque<usize> = VecDeque::new();
    let mut d = vec![-1; n];

    colors[0] = Color::Gray;
    queue.push_back(1);
    d[0] = 0;

    while let Some(id) = queue.pop_front() {
        for (i, &is_neighbor) in matrix[id - 1].iter().enumerate() {
            if is_neighbor {
                if let Color::White = colors[i] {
                    colors[i] = Color::Gray;
                    queue.push_back(i + 1);
                    d[i] = d[id - 1] + 1;
                }
            }
        }

        colors[id - 1] = Color::Black;
    }

    d.iter()
        .enumerate()
        .map(|(i, d)| (i + 1, *d))
        .collect::<Vec<(usize, i32)>>()
}

fn run(input: &str) -> String {
    let mut sc = Scanner::new(input);
    let n: usize = sc.next();
    // 一旦番号順に流れてくる前提
    let adjacency_lists: Vec<Vec<usize>> = (0..n)
        .map(|_| {
            sc.next::<usize>();
            let k: usize = sc.next();

            (0..k).map(|_| sc.next()).collect::<Vec<usize>>()
        })
        .collect();

    solve(n, &adjacency_lists)
        .iter()
        .map(|(id, d)| format!("{} {}\n", id, d))
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
5
1 2 2 4
2 1 4
3 0
4 1 3
5 1 2
";

        let output = "\
1 0
2 1
3 2
4 1
5 -1
";

        assert_eq!(run(input), output);
    }
}
