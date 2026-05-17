// ALDS1_2_A: Bubble Sort
// https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ALDS1_2_A
fn solve(values: &[i32]) -> (Vec<i32>, usize) {
    let mut values_vec = values.to_vec();
    let mut count = 0;

    for i in 0..(values_vec.len() - 1) {
        // 交換がなくなったタイミングで早期に break するため
        let mut was_swapped = false;
        for j in 0..(values_vec.len() - (i + 1)) {
            if values_vec[j] > values_vec[j + 1] {
                // swap method 便利だな
                values_vec.swap(j, j + 1);
                count += 1;

                was_swapped = true;
            }
        }

        if !was_swapped {
            break;
        }
    }

    (values_vec, count)
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::solve;

    #[test]
    fn sample_1() {
        assert_eq!(solve(&[5, 3, 2, 4, 1]), (vec![1, 2, 3, 4, 5], 8));
    }

    #[test]
    fn sample_2() {
        assert_eq!(solve(&[5, 2, 4, 6, 1, 3]), (vec![1, 2, 3, 4, 5, 6], 9));
    }
}
