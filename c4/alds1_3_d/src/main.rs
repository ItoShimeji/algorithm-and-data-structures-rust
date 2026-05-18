// ALDS1_3_D: Areas on the Cross-Section Diagram
// https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ALDS1_3_D
fn solve(diagram: &str) -> (usize, Vec<usize>) {
    let diagram = diagram.chars();
    let mut parts: Vec<Part> = Vec::new();

    let mut y = 0;
    for (i, token) in diagram.enumerate() {
        match token {
            '\\' => {
                parts.push(Part::Downhill { x: i, y });
                y -= 1;
            }
            '/' => {
                parts.push(Part::Uphill { x: i, y });
                y += 1;
            }
            '_' => parts.push(Part::Plane),
            _ => panic!(),
        }
    }

    let mut stack: Vec<Downhill> = Vec::new();
    let mut area: usize = 0;
    let mut partial_stack: Vec<(usize, usize)> = Vec::new();

    for part in parts {
        match part {
            Part::Downhill { x, y } => stack.push(Downhill { x, y }),
            Part::Uphill { x, y } => {
                if let Some(top_part) = stack.pop() {
                    if top_part.y == y + 1 {
                        let increment: usize = x - top_part.x;
                        let mut partial_area = increment;
                        area += increment;

                        while let Some(&(left, child_area)) = partial_stack.last() {
                            if left < top_part.x {
                                break;
                            }

                            partial_area += child_area;
                            partial_stack.pop();
                        }

                        partial_stack.push((top_part.x, partial_area));
                    } else {
                        panic!("top_part.y: {}, y: {}", top_part.y, y);
                    }
                }
            }
            _ => {}
        }
    }

    (
        area,
        partial_stack
            .into_iter()
            .map(|(_, partial_area)| partial_area)
            .collect(),
    )
}

struct Downhill {
    x: usize,
    y: isize,
}

// 座標は(0, 0)始まりで、Areaの始点を表す。
enum Part {
    Downhill { x: usize, y: isize },
    Uphill { x: usize, y: isize },
    Plane,
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::solve;

    #[test]
    fn sample_1() {
        assert_eq!(solve(r"\\//"), (4, vec![4]));
    }

    #[test]
    fn sample_2() {
        assert_eq!(
            solve(r"\\///\_/\/\\\\/_/\\///__\\\_\\/_\/_/\"),
            (35, vec![4, 2, 1, 19, 9])
        );
    }
}
