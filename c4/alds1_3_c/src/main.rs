use alds1_3_c::DoublyLinkedList;

// ALDS1_3_C: Doubly Linked List
// https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ALDS1_3_C
#[derive(Clone, Debug, PartialEq, Eq)]
enum Command {
    Insert(i64),
    Delete(i64),
    DeleteFirst,
    DeleteLast,
}

fn solve(commands: &[Command]) -> Vec<i64> {
    let _ = commands;
    // 標準ライブラリにも存在する
    // https://doc.rust-lang.org/beta/std/collections/struct.LinkedList.html
    let mut list: DoublyLinkedList<i64> = DoublyLinkedList::new();

    for command in commands {
        match command {
            Command::Insert(key) => list.push_front(*key),
            Command::Delete(key) => {
                list.delete(*key);
            }
            Command::DeleteFirst => {
                list.pop_front();
            }
            Command::DeleteLast => {
                list.pop_back();
            }
        }
    }

    list.to_vec()
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::{solve, Command};

    #[test]
    fn sample_1() {
        let commands = [
            Command::Insert(5),
            Command::Insert(2),
            Command::Insert(3),
            Command::Insert(1),
            Command::Delete(3),
            Command::Insert(6),
            Command::Delete(5),
        ];

        assert_eq!(solve(&commands), vec![6, 1, 2]);
    }

    #[test]
    fn sample_2() {
        let commands = [
            Command::Insert(5),
            Command::Insert(2),
            Command::Insert(3),
            Command::Insert(1),
            Command::Delete(3),
            Command::Insert(6),
            Command::Delete(5),
            Command::DeleteFirst,
            Command::DeleteLast,
        ];

        assert_eq!(solve(&commands), vec![1]);
    }
}
