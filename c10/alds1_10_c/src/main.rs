// ALDS1_10_C: Longest Common Subsequence
// https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ALDS1_10_C
use std::{
    cmp::max,
    io::{self, Read},
};

#[allow(dead_code)]
fn solve(x: &str, y: &str) -> usize {
    let x = x.as_bytes();
    let y = y.as_bytes();
    // 全て 0 で埋めておく
    // index 0 は matrix の外側
    let mut dp: Vec<Vec<usize>> = vec![vec![0; y.len() + 1]; x.len() + 1];

    for i in 1..=x.len() {
        for j in 1..=y.len() {
            if x[i - 1] == y[j - 1] {
                // 文字がマッチした時、それぞれの文字以前の LCS + 1 が長さになる
                dp[i][j] = dp[i - 1][j - 1] + 1;
            } else {
                // 文字がマッチしなかった場合、それぞれの文字がそれぞれ存在しなかった場合を考え、
                // それらの長い方を選ぶ
                dp[i][j] = max(dp[i - 1][j], dp[i][j - 1]);
            }
        }
    }

    dp[x.len()][y.len()]
}

fn run(input: &str) -> String {
    let _ = input;

    todo!("ここに入力のパースと出力の整形を実装する")
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
        assert_eq!(run("3\nabcbdab\nbdcaba\nabc\nabc\nabc\nbc\n"), "4\n3\n2\n");
    }
}
