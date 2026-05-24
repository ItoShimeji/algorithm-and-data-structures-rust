// ALDS1_9_B: Maximum Heap
// https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ALDS1_9_B
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
fn solve(values: &[i32]) -> Vec<i32> {
    let mut heap: Vec<i32> = values.to_vec();
    let length = heap.len();

    // length / 2 とすることで heap 上の葉の一個上の node から走査でき、効率が良い
    // index を反転させているのは、heap を下から完成させ、max_heapify の前提に則るため
    // i == 1 は root
    // ここで O(n) でループを回しており、max_heapify の計算量は (0/2 + 1/4 + 2/8 + 3/16 + ...) と定数に収束するため、
    // 全体で O(n)
    for i in (1..=length / 2).rev() {
        max_heapify(&mut heap, i as i32);
    }

    heap
}

fn max_heapify(values: &mut Vec<i32>, i: i32) {
    let left = values.get((left_i(i) - 1) as usize).copied();
    let right = values.get((right_i(i) - 1) as usize).copied();

    // この時点で、i より下の木構造は max heap になっている前提
    let mut largest: i32 = i;
    if let Some(left_value) = left {
        if values[(i - 1) as usize] < left_value {
            largest = left_i(i);
        }
    }
    if let Some(right_value) = right {
        if values[(largest - 1) as usize] < right_value {
            largest = right_i(i);
        }
    }

    if largest != i {
        values.swap((i - 1) as usize, (largest - 1) as usize);
        // largest は現在 i が存在している
        max_heapify(values, largest);
    }
}

fn left_i(i: i32) -> i32 {
    2 * i
}

fn right_i(i: i32) -> i32 {
    2 * i + 1
}

fn run(input: &str) -> String {
    let mut sc = Scanner::new(input);
    let n: usize = sc.next();
    let values: Vec<i32> = (0..n).map(|_| sc.next()).collect();

    let heap = solve(&values);

    // 空白区切りで出力
    let mut heap = heap.iter().map(|n| format!(" {n}")).collect::<String>();
    heap.push('\n');
    heap
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
10
4 1 3 2 16 9 10 14 8 7
";

        assert_eq!(run(input), " 16 14 10 8 7 9 3 2 4 1\n");
    }
}
