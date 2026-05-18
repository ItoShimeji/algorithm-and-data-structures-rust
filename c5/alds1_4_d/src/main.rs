// ALDS1_4_D: Allocation
// https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ALDS1_4_D
fn solve(trucks: usize, weights: &[u64]) -> u64 {
    let sum_weights: usize = weights.iter().map(|w| *w as usize).sum();

    // weight ごとの鳩の巣原理(の一般化？)より、積載量の合計の平均以上のトラックは
    // 必ず1つは存在するため、これをひとまず、max_capacity とする。
    let mut max_capacity = sum_weights / trucks
        + if sum_weights.is_multiple_of(trucks) {
            1
        } else {
            0
        };

    // 必要なトラックの数
    let mut track_count = 0;

    while track_count != trucks {
        // あるトラックに現在積載している量
        let mut loading = 0;

        for &weight in weights {
            if loading + weight as usize <= max_capacity {
                loading += max_capacity;
            } else {
                track_count += 1;
                loading = 0;
            }
        }

        if loading > 0 {
            track_count += 1;
        }

        max_capacity += 1;
    }

    max_capacity as u64
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
}
