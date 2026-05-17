// ALDS1_3_A: Stack
// https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ALDS1_3_A
fn solve(tokens: &[&str]) -> i64 {
    let mut stack: Vec<i64> = Vec::with_capacity(tokens.len());

    for token in tokens {
        match *token {
            "+" => {
                // 演算によっては項の順番が大事なので、わかりやすい名前で取り出す。
                let rhs = stack.pop().unwrap();
                let lhs = stack.pop().unwrap();
                stack.push(lhs + rhs);
            }
            "-" => {
                let rhs = stack.pop().unwrap();
                let lhs = stack.pop().unwrap();
                stack.push(lhs - rhs);
            }
            "*" => {
                let rhs = stack.pop().unwrap();
                let lhs = stack.pop().unwrap();
                stack.push(lhs * rhs);
            }
            "/" => {
                let rhs = stack.pop().unwrap();
                let lhs = stack.pop().unwrap();
                stack.push(lhs / rhs);
            }
            _ => stack.push(token.parse().unwrap()),
        }
    }

    stack.pop().unwrap()
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::solve;

    #[test]
    fn sample_1() {
        assert_eq!(solve(&["1", "2", "+"]), 3);
    }

    #[test]
    fn sample_2() {
        assert_eq!(solve(&["1", "2", "+", "3", "4", "-", "*"]), -3);
    }
}
