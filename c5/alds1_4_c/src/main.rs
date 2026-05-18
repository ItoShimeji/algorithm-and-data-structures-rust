use alds1_4_c::HashMap;

// ALDS1_4_C: Search III
// https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ALDS1_4_C
#[derive(Clone, Debug, PartialEq, Eq)]
enum Command<'a> {
    Insert(&'a str),
    Find(&'a str),
}

fn solve(commands: &[Command]) -> Vec<bool> {
    let mut map: HashMap<String> = HashMap::new(5, 4);
    let mut found_items: Vec<bool> = Vec::new();

    for command in commands {
        match command {
            Command::Insert(key) => map.insert(key, key.to_string()),
            Command::Find(key) => {
                found_items.push(map.find(key).is_some());
            }
        }
    }

    found_items
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::{solve, Command};

    #[test]
    fn sample_1() {
        let commands = [
            Command::Insert("A"),
            Command::Insert("T"),
            Command::Insert("C"),
            Command::Find("G"),
            Command::Find("A"),
        ];

        assert_eq!(solve(&commands), vec![false, true]);
    }

    #[test]
    fn sample_2() {
        let commands = [
            Command::Insert("AAA"),
            Command::Insert("AAC"),
            Command::Insert("AGA"),
            Command::Insert("AGG"),
            Command::Insert("TTT"),
            Command::Find("AAA"),
            Command::Find("CCC"),
            Command::Find("CCC"),
            Command::Insert("CCC"),
            Command::Find("CCC"),
            Command::Insert("T"),
            Command::Find("TTT"),
            Command::Find("T"),
        ];

        assert_eq!(solve(&commands), vec![true, false, false, true, true, true]);
    }
}
