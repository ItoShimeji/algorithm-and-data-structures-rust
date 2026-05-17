// ALDS1_2_D: Shell Sort
// https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ALDS1_2_D
fn solve(values: &[i64]) -> (Vec<usize>, usize, Vec<i64>) {
    let mut values = values.to_vec();
    let mut count = 0;

    let h_list = if values.len() > 3 {
        vec![4, 1]
    } else {
        vec![1]
    };

    for h in &h_list {
        let (new_values, new_count) = insertion_sort(&mut values, *h);
        values = new_values.clone();
        count += new_count;
    }

    (h_list, count, values)
}

fn insertion_sort(values: &mut Vec<i64>, h: usize) -> (&Vec<i64>, usize) {
    let mut count: usize = 0;

    for i in 1..values.len() {
        let v = values[i];
        let mut j = i;

        while j >= h && values[j - h] > v {
            values[j] = values[j - h];
            j -= h;

            count += 1;
        }

        values[j] = v;
    }

    (values, count)
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::solve;

    #[test]
    fn sample_1() {
        assert_eq!(
            solve(&[5, 1, 4, 3, 2]),
            (vec![4, 1], 3, vec![1, 2, 3, 4, 5])
        );
    }

    #[test]
    fn sample_2() {
        assert_eq!(solve(&[3, 2, 1]), (vec![1], 3, vec![1, 2, 3]));
    }
}
