// ALDS1_12_B: Single Source Shortest Path I
// https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ALDS1_12_B
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
    let mut current_v = 0;
    let mut used = vec![false; n];
    used[0] = true;
    let mut d = vec![i32::MAX; n];
    d[0] = 0;
    // この実装では使用していないが、逆から走査することで任意の点から始点への最短経路がわかる。
    let mut p: Vec<Option<usize>> = vec![None; n];

    loop {
        used[current_v] = true;

        for edge in &graph[current_v] {
            if d[current_v] + edge.cost < d[edge.to] {
                d[edge.to] = d[current_v] + edge.cost;
                p[edge.to] = Some(current_v);
            }
        }

        // 現在の未確定頂点から距離が最小の頂点を選ぶ
        // その頂点への経路はこれで確定。
        let next_v = (1..n).filter(|i| !used[*i]).min_by_key(|i| d[*i]);
        match next_v {
            Some(next_v) => current_v = next_v,
            None => break,
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
