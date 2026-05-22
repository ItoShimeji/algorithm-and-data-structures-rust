// ALDS1_5_D: The Number of Inversions
// https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ALDS1_5_D
fn solve(values: &[i32]) -> usize {
    let values = &mut values.to_vec();
    let length = values.len();

    merge_sort(values, 0, length)
}

fn merge_sort(values: &mut [i32], left: usize, right: usize) -> usize {
    if left + 1 >= right {
        return 0;
    }
    let mid = left + (right - left) / 2;

    let mut count = 0;

    count += merge_sort(values, left, mid);
    count += merge_sort(values, mid, right);
    count += merge(values, left, mid, right);

    count
}

fn merge(values: &mut [i32], left: usize, mid: usize, right: usize) -> usize {
    // データ構造は vec deque の方が良いのかもしれない
    let left_values = &mut values[left..mid].to_vec();
    let right_values = &mut values[mid..right].to_vec();

    // 番兵を追加
    left_values.push(i32::MAX);
    right_values.push(i32::MAX);

    let mut head_left = 0;
    let mut head_right = 0;

    let mut count = 0;

    for index in left..right {
        // 同じ値の際に、left を優先することで安定ソートにする
        if left_values[head_left] <= right_values[head_right] {
            values[index] = left_values[head_left];
            head_left += 1;
        } else {
            values[index] = right_values[head_right];
            head_right += 1;
            // left 配列に残っている要素数
            count += (mid - left) - head_left;
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
        assert_eq!(solve(&[3, 5, 2, 1, 4]), 6);
    }
}
