// ALDS1_2_B: Selection Sort
// https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ALDS1_2_B
fn solve(values: &[i32]) -> (Vec<i32>, usize) {
    let mut values = values.to_vec();
    let mut count = 0;

    for i in 0..(values.len() - 1) {
        let mut min_index = i;
        for j in i..values.len() {
            min_index = if values[j] < values[min_index] {
                j
            } else {
                min_index
            };
        }

        if i != min_index {
            values.swap(i, min_index);
            count += 1;
        }
    }

    (values, count)
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::solve;

    #[test]
    fn sample_1() {
        assert_eq!(solve(&[5, 6, 4, 2, 1, 3]), (vec![1, 2, 3, 4, 5, 6], 4));
    }

    #[test]
    fn sample_2() {
        assert_eq!(solve(&[5, 2, 4, 6, 1, 3]), (vec![1, 2, 3, 4, 5, 6], 3));
    }
}
