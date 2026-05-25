// ALDS1_10_B: Matrix-chain Multiplication
// https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ALDS1_10_B
use std::{
    cmp::min,
    io::{self, Read},
    usize,
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

fn multiplication_count(lhs: (usize, usize), rhs: (usize, usize)) -> usize {
    lhs.0 * lhs.1 * rhs.1
}

struct Dp {
    count: usize,
    matrix: (usize, usize),
}

#[allow(dead_code)]
fn solve(matrix_sizes: &[(usize, usize)]) -> usize {
    // index 0: 0 <- 一個だけ行列が与えられた場合
    // index 1: l * m * n <- 1つ目と2つ目の積の際の乗算の回数
    // (乗算回数, (その時の乗算結果の行列の次元))
    let mut dp: Vec<Dp> = Vec::new();
    dp.push(Dp {
        count: 0,
        matrix: matrix_sizes[0],
    });

    for i in 1..matrix_sizes.len() {
        let first = (matrix_sizes[0].0, matrix_sizes[i].1);
        let neighbor = (matrix_sizes[i - 1].0, matrix_sizes[i].1);
        let a = multiplication_count(dp[i - 1].matrix, matrix_sizes[i]);
        let b = if i == 1 {
            usize::MAX
        } else {
            multiplication_count(dp[i - 2].matrix, neighbor)
        };

        dp.push(Dp {
            count: min(a, b),
            matrix: first,
        });
    }

    dp.last().unwrap().count
}

fn run(input: &str) -> String {
    let mut sc = Scanner::new(input);
    let n: usize = sc.next();
    let matrix_sizes: Vec<(usize, usize)> = (0..n).map(|_| (sc.next(), sc.next())).collect();

    let mut output = solve(&matrix_sizes).to_string();
    output.push('\n');
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
        assert_eq!(
            run("6\n30 35\n35 15\n15 5\n5 10\n10 20\n20 25\n"),
            "15125\n"
        );
    }
}
