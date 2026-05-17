use std::collections::VecDeque;

// ALDS1_3_B: Queue
// https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ALDS1_3_B
#[derive(Clone, Debug, PartialEq, Eq)]
struct Process<'a> {
    name: &'a str,
    time: u32,
}

fn solve<'a>(quantum: u32, processes: &[Process<'a>]) -> Vec<(&'a str, u32)> {
    // 先頭の取り出しが多くなるため、VecDequeを使用する。
    // 今回は要素数が変わらないため、headを動かしながら、値を変えていくような
    // データ構造を独自実装した方が実行効率が良いが、ここでは、標準ライブラリを使用する。
    let mut processes: VecDeque<Process> = processes.iter().cloned().collect();
    let mut result: Vec<(&'a str, u32)> = Vec::with_capacity(processes.len());

    let mut total_time = 0;

    while !processes.is_empty() {
        let mut process = processes.pop_front().unwrap();

        let time = process.time;
        if time <= quantum {
            total_time += process.time;
            process.time = 0;

            result.push((process.name, total_time));
        } else {
            total_time += quantum;
            process.time -= quantum;
            processes.push_back(process);
        }
    }

    result
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::{solve, Process};

    #[test]
    fn sample_1() {
        let processes = [
            Process {
                name: "p1",
                time: 150,
            },
            Process {
                name: "p2",
                time: 80,
            },
            Process {
                name: "p3",
                time: 200,
            },
            Process {
                name: "p4",
                time: 350,
            },
            Process {
                name: "p5",
                time: 20,
            },
        ];

        assert_eq!(
            solve(100, &processes),
            vec![
                ("p2", 180),
                ("p5", 400),
                ("p1", 450),
                ("p3", 550),
                ("p4", 800)
            ]
        );
    }
}
