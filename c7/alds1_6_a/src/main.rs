// ALDS1_6_A: Counting Sort
// https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ALDS1_6_A
fn solve(_values: &[usize]) -> Vec<usize> {
    let &max = _values.iter().max().unwrap();

    let mut counts: Vec<usize> = (0..=max).map(|_| 0).collect();

    for &v in _values {
        counts[v] += 1;
    }

    let mut sum_count: Vec<usize> = (0..=max).map(|_| 0).collect();

    for (index, &count) in counts.iter().enumerate() {
        sum_count[index] = count + if index > 0 { sum_count[index - 1] } else { 0 };
    }

    let mut sorted: Vec<usize> = (0.._values.len()).map(|_| 0).collect();

    for &v in _values.iter().rev() {
        sorted[sum_count[v] - 1] = v;
        sum_count[v] -= 1;
    }

    sorted
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::solve;

    #[test]
    fn sample_1() {
        assert_eq!(solve(&[2, 5, 1, 3, 2, 3, 0]), vec![0, 1, 2, 2, 3, 3, 5]);
    }
}
