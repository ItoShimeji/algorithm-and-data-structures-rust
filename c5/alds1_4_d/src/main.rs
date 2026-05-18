// ALDS1_4_D: Allocation
// https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ALDS1_4_D
fn solve(trucks: usize, weights: &[u64]) -> u64 {
    let sum_weights: u64 = weights.iter().sum();
    let max_weight = weights.iter().copied().max().unwrap_or(0);

    // できるだけ少ない回数で capacity を当てるのではなく、より一般に
    // 高速な二分探索を使う。
    let mut lower = max_weight;
    let mut upper = sum_weights;

    while lower < upper {
        let capacity = lower + (upper - lower) / 2;

        if can_load(capacity, trucks, weights) {
            upper = capacity;
        } else {
            lower = capacity + 1;
        }
    }

    lower
}

fn can_load(capacity: u64, trucks: usize, weights: &[u64]) -> bool {
    let mut truck_count = 1;
    let mut loading = 0;

    for &weight in weights {
        if loading + weight <= capacity {
            loading += weight;
        } else {
            truck_count += 1;
            loading = weight;
        }

        if truck_count > trucks {
            return false;
        }
    }

    true
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::solve;

    #[test]
    fn sample_1() {
        assert_eq!(solve(3, &[8, 1, 7, 3, 9]), 10);
    }

    #[test]
    fn sample_2() {
        assert_eq!(solve(2, &[1, 2, 2, 6]), 6);
    }

    #[test]
    fn small_case() {
        assert_eq!(solve(1, &[1, 1]), 2);
    }
}
