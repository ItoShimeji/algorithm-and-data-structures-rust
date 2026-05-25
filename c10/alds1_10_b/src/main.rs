// ALDS1_10_B: Matrix-chain Multiplication
// https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ALDS1_10_B
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
fn solve(matrix_sizes: &[(usize, usize)]) -> usize {
    let n = matrix_sizes.len();
    let mut dp = vec![vec![0usize; n]; n];

    // l, .., k, .., r
    // かっこで作られる部分の長さを 2 から n（全体）まで
    for len in 2..=n {
        // 部分の左を動かしていく
        for l in 0..=n - len {
            let r = l + len - 1;
            dp[l][r] = usize::MAX;

            // 部分の中でどこで分割するか
            // (M_1, M_2, M_3)(M_4, M_5) なら、k = 3
            for k in l..r {
                let cost = dp[l][k]
                    + dp[k + 1][r]
                    + matrix_sizes[l].0 * matrix_sizes[k].1 * matrix_sizes[r].1;

                dp[l][r] = dp[l][r].min(cost);
            }
        }
    }

    dp[0][n - 1]
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
