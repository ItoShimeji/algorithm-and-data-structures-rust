// ALDS1_12_A: Minimum Spanning Tree
// https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ALDS1_12_A
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

// enum Color {
//     White,
//     Gray,
//     Black,
// }

#[allow(dead_code)]
fn solve(n: usize, adjacency_matrix: &[Vec<i32>]) -> i32 {
    let mut current_v = 0;
    let mut used = vec![false; n];
    used[0] = true;
    let mut min_cost = vec![i32::MAX; n];
    min_cost[0] = 0;

    loop {
        used[current_v] = true;

        for (i, &a) in adjacency_matrix[current_v].iter().enumerate() {
            if !used[i] && a < min_cost[i] {
                min_cost[i] = a;
            }
        }

        // 使用されていない頂点から最小の重みを持つ辺で移動できるものを探す。
        // 1つも無い = 探索終了 でループ脱出
        let next_v = (1..n).filter(|i| !used[*i]).min_by_key(|i| min_cost[*i]);
        match next_v {
            Some(next_v) => current_v = next_v,
            None => break,
        }
    }

    // もし全域木が存在しないならば、min_cost で i32::MAX から更新されていない要素が生じ、
    // ここでオーバーフローする
    min_cost.iter().sum()
}

fn run(input: &str) -> String {
    let mut sc = Scanner::new(input);
    let n: usize = sc.next();

    let adjacency_matrix = (0..n)
        .map(|_| {
            (0..n)
                .map(|_| sc.next::<i32>())
                .map(|x| if x == -1 { i32::MAX } else { x })
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();

    format!("{}\n", solve(n, &adjacency_matrix))
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
-1 2 3 1 -1
2 -1 -1 4 -1
3 -1 -1 1 1
1 4 1 -1 3
-1 -1 1 3 -1
";

        assert_eq!(run(input), "5\n");
    }
}
