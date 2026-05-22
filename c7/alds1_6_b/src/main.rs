// ALDS1_6_B: Partition
// https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ALDS1_6_B
fn solve(values: &[usize]) -> (Vec<usize>, usize) {
    quick_sort(&mut values);


}

fn quick_sort(values: &mut [usize]) {
    if values.len() < 2 {
        return;
    }

    let pivot = median_of_three(0, values.len() / 2 + 1, values.len() - 1);
    let value_pivot = values[pivot];

    // 左右から走査していく head
    let mut head_left = 0;
    let mut head_right = values.len() - 1;

    // 左右領域を分割した場合の右領域の最初のインデックス
    let mut index_border = 0;

    // 左右から head を狭めていき、左右領域に分割し、境界を決める
    loop {
        if value_pivot < values[head_left] && values[head_right] <= value_pivot {
            // 大小が左右の領域で逆になっているため、swap
            values.swap(head_left, head_right);

            head_left += 1;
            head_right -= 1;
        } else {
            // 大小が左右の領域に沿っている場合は head をずらす
            if values[head_left] <= value_pivot {
                head_left += 1;
            }
            if value_pivot < values[head_right] {
                head_right -= 1;
            }
        }

        if head_left + 1 == head_right {
            // ..., left head, right head, ...

            index_border = head_right;
            break;
        } else if head_left == head_right {
            // ..., left head = right head, ...

            index_border = head_left + if head_left >= value_pivot {0} else {1};
            break;
        }
    }

    quick_sort(&mut values[0..index_border]);
    quick_sort(&mut values[index_border..]);
}

fn median_of_three(a: usize, b: usize, c: usize) -> usize {
    if (a <= b && b <= c) || (c <= b && b <= a) {
        b
    } else if (b <= a && a <= c) || (c <= a && a <= b) {
        a
    } else {
        c
    }
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
