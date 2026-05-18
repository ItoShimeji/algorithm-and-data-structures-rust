// ALDS1_4_A: Search I
// https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ALDS1_4_A
fn solve(values: &[u64], queries: &[u64]) -> usize {
    let mut count = 0;

    // 模範解答にある番兵は for in 文法の Rust では不要。
    for q in queries {
        for v in values {
            if q == v {
                count += 1;
                break;
            }
        }
    }

    count
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::solve;

    #[test]
    fn sample_1() {
        assert_eq!(solve(&[1, 2, 3, 4, 5], &[3, 4, 1]), 3);
    }

    #[test]
    fn sample_2() {
        assert_eq!(solve(&[3, 1, 2], &[5]), 0);
    }

    #[test]
    fn sample_3() {
        assert_eq!(solve(&[1, 1, 2, 2, 3], &[1, 2]), 2);
    }
}
