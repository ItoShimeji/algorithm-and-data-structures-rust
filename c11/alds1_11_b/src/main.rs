// ALDS1_11_B: Depth First Search
// https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ALDS1_11_B
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

#[derive(Clone, Copy)]
struct Vertex {
    id: usize,
    d: Option<usize>,
    f: Option<usize>,
}

impl Vertex {
    fn tuple(&self) -> (usize, usize, usize) {
        (self.id, self.d.unwrap(), self.f.unwrap())
    }
}

#[allow(dead_code)]
fn solve(n: usize, adjacency_lists: &[Vec<usize>]) -> Vec<(usize, usize, usize)> {
    let mut vertex_list: Vec<Vertex> = (0..n)
        .map(|i| Vertex {
            id: i + 1,
            d: None,
            f: None,
        })
        .collect();

    for i in 0..adjacency_lists.len() {
        dfs(adjacency_lists, &mut vertex_list, i, 0);
    }

    vertex_list.iter().map(|v| v.tuple()).collect()
}

fn dfs(
    adjacency_lists: &[Vec<usize>],
    vertex_list: &mut Vec<Vertex>,
    i: usize,
    time: usize,
) -> usize {
    if vertex_list[i].d.is_some() {
        return time;
    }
    // ある頂点に進むタイミングで +1
    let mut time = time + 1;
    vertex_list[i].d = Some(time);

    for j in 0..adjacency_lists[i].len() {
        let v = adjacency_lists[i][j];
        time = dfs(adjacency_lists, vertex_list, v - 1, time);
    }

    // 前の頂点に戻るタイミングで +1
    time += 1;
    vertex_list[i].f = Some(time);

    time
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
        .map(|v| format!("{} {} {}\n", v.0, v.1, v.2))
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
6
1 2 2 3
2 2 3 4
3 1 5
4 1 6
5 1 6
6 0
";

        let output = "\
1 1 12
2 2 11
3 3 8
4 9 10
5 4 7
6 5 6
";

        assert_eq!(run(input), output);
    }
}
