// ALDS1_11_D: Connected Components
// https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ALDS1_11_D
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

#[allow(dead_code)]
fn solve(n: usize, relations: &[(usize, usize)], queries: &[(usize, usize)]) -> Vec<bool> {
    // index: 頂点番号
    let mut graph: Vec<Vec<usize>> = vec![Vec::new(); n];
    for &(s, t) in relations {
        graph[s].push(t);
        graph[t].push(s);
    }

    let mut components: Vec<Option<usize>> = vec![None; n];

    for i in 0..n {
        if components[i].is_some() {
            continue;
        }
        components[i] = Some(i);
        let mut queue: VecDeque<usize> = VecDeque::new();
        queue.push_back(i);
        while let Some(j) = queue.pop_front() {
            for &to in &graph[j] {
                if components[to].is_none() {
                    queue.push_back(to);
                    // ここでは component で差がつけば良いだけのため、
                    // bfs の始点の番号を追加
                    components[to] = Some(i);
                }
            }
        }
    }

    queries
        .iter()
        .map(|&(s, t)| components[s] == components[t])
        .collect()
}

fn run(input: &str) -> String {
    let mut sc = Scanner::new(input);
    let n: usize = sc.next();
    let m: usize = sc.next();
    let mut relations: Vec<(usize, usize)> = Vec::new();

    for _ in 0..m {
        let s: usize = sc.next();
        let t: usize = sc.next();
        relations.push((s, t));
    }

    let q: usize = sc.next();
    let mut queries: Vec<(usize, usize)> = Vec::new();

    for _ in 0..q {
        let s: usize = sc.next();
        let t: usize = sc.next();
        queries.push((s, t));
    }

    solve(n, &relations, &queries)
        .iter()
        .map(|b| {
            if *b {
                "yes".to_string()
            } else {
                "no".to_string()
            }
        })
        .map(|s| format!("{s}\n"))
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
10 9
0 1
0 2
3 4
5 7
5 6
6 7
6 8
7 8
8 9
3
0 1
5 9
1 3
";

        let output = "\
yes
yes
no
";

        assert_eq!(run(input), output);
    }
}
