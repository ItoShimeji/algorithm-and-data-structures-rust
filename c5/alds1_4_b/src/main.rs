// ALDS1_4_B: Search II
// https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ALDS1_4_B
fn solve(sorted_values: &[u64], queries: &[u64]) -> usize {
    let queries = queries.to_vec();
    let mut count = 0;

    for q in queries {
        // start を含む
        let mut start_index = 0;
        // end は含まない
        let mut end_index = sorted_values.len();

        while start_index < end_index {
            let median_index = (end_index + start_index) / 2;
            let median = sorted_values[median_index];

            if q > median {
                start_index = median_index + 1;
            } else if q == median {
                count += 1;
                break;
            } else {
                end_index = median_index;
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
        assert_eq!(solve(&[1, 2, 3], &[5]), 0);
    }

    #[test]
    fn sample_3() {
        assert_eq!(solve(&[1, 1, 2, 2, 3], &[1, 2]), 2);
    }
}
