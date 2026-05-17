// ALDS1_1_A: Insertion Sort
// https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ALDS1_1_A
fn solve(values: &[i32]) -> Vec<Vec<i32>> {
    let mut result: Vec<Vec<i32>> = Vec::new();
    let mut values_vec = values.to_vec();
    result.push(values_vec.clone());

    for i in 1..values_vec.len() {
        let v = values_vec[i];
        let mut j = i;

        while j > 0 && values_vec[j - 1] > v {
            values_vec[j] = values_vec[j - 1];
            j -= 1;
        }

        values_vec[j] = v;
        result.push(values_vec.clone());
    }

    result
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::solve;

    #[test]
    fn sample_1() {
        assert_eq!(
            solve(&[5, 2, 4, 6, 1, 3]),
            vec![
                vec![5, 2, 4, 6, 1, 3],
                vec![2, 5, 4, 6, 1, 3],
                vec![2, 4, 5, 6, 1, 3],
                vec![2, 4, 5, 6, 1, 3],
                vec![1, 2, 4, 5, 6, 3],
                vec![1, 2, 3, 4, 5, 6],
            ]
        );
    }
}
