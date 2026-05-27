// ALDS1_12_C: Single Source Shortest Path II
// https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ALDS1_12_C
use std::cmp::Reverse;
use std::collections::BinaryHeap;
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
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Edge {
    to: usize,
    cost: i32,
}

#[allow(dead_code)]
fn solve(n: usize, graph: &[Vec<Edge>]) -> Vec<i32> {
    let mut d = vec![i32::MAX; n];
    d[0] = 0;

    // i32: cost
    // usize: 頂点
    // tuple は 第一要素から順に大小比較される
    let mut heap: BinaryHeap<Reverse<(i32, usize)>> = BinaryHeap::new();
    heap.push(Reverse((0, 0)));

    while let Some(Reverse(v)) = heap.pop() {
        // ここで break してはいけない
        // より現在の距離が大きくて探索中のものが存在する可能性がある
        if d[v.1] < v.0 {
            continue;
        }

        for edge in &graph[v.1] {
            if d[v.1] + edge.cost < d[edge.to] {
                d[edge.to] = d[v.1] + edge.cost;
                // 古い要素は無視して投入だけ行う
                heap.push(Reverse((d[edge.to], edge.to)));
            }
        }
    }

    d
}

fn run(input: &str) -> String {
    let mut sc = Scanner::new(input);
    let n: usize = sc.next();

    let input = (0..n)
        .map(|_| {
            let u: usize = sc.next();
            let k: usize = sc.next();

            let list = (0..k)
                .map(|_| Edge {
                    to: sc.next(),
                    cost: sc.next(),
                })
                .collect::<Vec<Edge>>();

            (u, list)
        })
        .collect::<Vec<(usize, Vec<Edge>)>>();

    let mut graph: Vec<Vec<Edge>> = vec![Vec::new(); n];

    for (u, edge_list) in input {
        graph[u] = edge_list;
    }

    solve(n, &graph)
        .iter()
        .enumerate()
        .map(|(i, &d)| format!("{} {}\n", i, d))
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
0 3 2 3 3 1 1 2
1 2 0 2 3 4
2 3 0 3 3 1 4 1
3 4 2 1 0 1 1 4 4 3
4 2 2 1 3 3
";

        let output = "\
0 0
1 2
2 2
3 1
4 3
";

        assert_eq!(run(input), output);
    }
}
