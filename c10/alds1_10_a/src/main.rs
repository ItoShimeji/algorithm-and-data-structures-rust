// ALDS1_10_A: Fibonacci Number
// https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ALDS1_10_A
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
fn solve(n: usize) -> usize {
    if let 0..=1 = n {
        return 1;
    }

    let mut fibonacci: Vec<usize> = vec![1; n + 1];

    for i in 2..=n {
        fibonacci[i] = fibonacci[i - 1] + fibonacci[i - 2];
    }

    *fibonacci.last().unwrap()
}

fn run(input: &str) -> String {
    let n: usize = Scanner::new(input).next();

    let mut output = solve(n).to_string();
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
        assert_eq!(run("3\n"), "3\n");
    }
}
