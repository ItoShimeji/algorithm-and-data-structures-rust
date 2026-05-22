// ALDS1_6_B: Partition
// https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ALDS1_6_B
fn solve(_values: &[usize]) -> (Vec<usize>, usize) {
    let mut values = _values.to_vec();
    let partition_value = _values[_values.len() - 1];

    // それぞれの領域の最大インデックス + 1
    let mut left_end = 0;
    let mut right_end = 0;

    for i in 0..(values.len() - 1) {
        // 問題文の疑似コードに従って、イコールをつける
        if values[i] <= partition_value {
            values.swap(left_end, right_end);

            left_end += 1;
            right_end += 1;
        } else {
            right_end += 1;
        }
    }

    // 最後に partition を right の先頭を交換
    values.swap(left_end, right_end);

    (values, left_end)
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::solve;

    #[test]
    fn sample_1() {
        assert_eq!(
            solve(&[13, 19, 9, 5, 12, 8, 7, 4, 21, 2, 6, 11]),
            (vec![9, 5, 8, 7, 4, 2, 6, 11, 21, 13, 19, 12], 7)
        );
    }
}
