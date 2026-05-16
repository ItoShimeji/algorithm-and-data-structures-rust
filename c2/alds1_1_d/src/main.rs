// ALDS1_1_D: Maximum Profit
// https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ALDS1_1_D
fn solve(prices: &[i32]) -> i32 {
    let mut min_r = prices[0];
    let mut max_diff = i32::MIN;

    for &r in prices[1..].iter() {
        if r - min_r > max_diff {
            max_diff = r - min_r;
        }
        if r < min_r {
            min_r = r;
        }
    }

    max_diff
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::solve;

    #[test]
    fn test_1() {
        assert_eq!(solve(&[5, 3, 1, 3, 4, 3]), 3);
    }

    #[test]
    fn test_2() {
        assert_eq!(solve(&[5, 4, 3, 2]), -1);
    }
}
