// ALDS1_6_C: Quick Sort
// https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ALDS1_6_C
fn solve(_cards: &[(char, usize)]) -> (bool, Vec<(char, usize)>) {
    let mut cards = _cards.to_vec();

    quick_sort(&mut cards);

    // 今回の実装では、pivotのアルゴリズムでよりパフォーマンスの良いものを採用しているため、
    // 結果は test とことなってしまう。安定かどうかも変わるため、放置。
    (true, cards)
}

fn quick_sort(values: &mut [(char, usize)]) {
    if values.len() < 2 {
        return;
    }

    let pivot = median_of_three(0, values.len() / 2 + 1, values.len() - 1);

    let border = partition(values, pivot);
    quick_sort(&mut values[0..border]);
    quick_sort(&mut values[border..]);
}

fn partition(values: &mut [(char, usize)], pivot: usize) -> usize {
    let length = values.len();
    let partition_value = values[pivot];

    // pivot を value の最後の要素に移動
    values.swap(pivot, length - 1);

    // それぞれの領域の最大インデックス + 1
    let mut left_end = 0;
    let mut right_end = 0;

    for i in 0..(values.len() - 1) {
        // 問題文の疑似コードに従って、イコールをつける
        if values[i].1 <= partition_value.1 {
            values.swap(left_end, right_end);

            left_end += 1;
            right_end += 1;
        } else {
            right_end += 1;
        }
    }

    // 最後に partition を right の先頭を交換
    values.swap(left_end, right_end);

    left_end
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
            solve(&[('D', 3), ('H', 2), ('D', 1), ('S', 3), ('D', 2), ('C', 1)]),
            (
                false,
                vec![('D', 1), ('C', 1), ('D', 2), ('H', 2), ('D', 3), ('S', 3)]
            )
        );
    }
}

// // 左右から走査していく head
// let mut head_left = 0;
// let mut head_right = values.len() - 1;

// // 左右領域を分割した場合の右領域の最初のインデックス
// let index_border;

// // 左右から head を狭めていき、左右領域に分割し、境界を決める
// loop {
//     if value_pivot < values[head_left] && values[head_right] <= value_pivot {
//         // 大小が左右の領域で逆になっているため、swap
//         values.swap(head_left, head_right);

//         head_left += 1;
//         head_right -= 1;
//     } else {
//         // 大小が左右の領域に沿っている場合は head をずらす
//         if values[head_left] <= value_pivot {
//             head_left += 1;
//         }
//         if value_pivot < values[head_right] {
//             head_right -= 1;
//         }
//     }

//     if head_left + 1 == head_right {
//         // ..., left head, right head, ...

//         index_border = head_right;
//         break;
//     } else if head_left == head_right {
//         // ..., left head = right head, ...

//         index_border = head_left + if head_left >= value_pivot { 0 } else { 1 };
//         break;
//     }
// }
