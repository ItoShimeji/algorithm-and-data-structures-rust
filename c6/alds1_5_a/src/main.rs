// ALDS1_5_A: Exhaustive Search
// https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ALDS1_5_A
fn solve(values: &[i32], queries: &[i32]) -> Vec<bool> {
    let mut result: Vec<bool> = Vec::new();

    for &query in queries {
        result.push(search(values, 0, query));
    }

    result
}

fn search(values: &[i32], current_value: i32, query_value: i32) -> bool {
    if values.is_empty() {
        return current_value == query_value;
    };

    // values の先頭を使用する・しないでそれぞれ再帰呼び出し
    search(&values[1..], current_value + values[0], query_value)
        || search(&values[1..], current_value, query_value)
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::solve;

    #[test]
    fn sample_1() {
        assert_eq!(
            solve(&[1, 5, 7, 10, 21], &[2, 4, 17, 8, 22, 21, 100, 35]),
            vec![false, false, true, true, true, true, false, false]
        );
    }
}
