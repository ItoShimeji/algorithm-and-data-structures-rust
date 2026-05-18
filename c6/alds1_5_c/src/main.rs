use std::ops::Add;
use std::ops::Mul;
use std::ops::Sub;

// ALDS1_5_C: Koch Curve
// https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ALDS1_5_C
fn solve(_depth: usize) -> Vec<(f64, f64)> {
    let p1 = Vertex { x: 0.0, y: 0.0 };
    let p2 = Vertex { x: 1.0, y: 0.0 };

    let mut vertices: Vec<Vertex> = Vec::new();
    vertices.push(p1.clone());
    vertices.extend(kock(&p1, &p2, 0, _depth));
    vertices.push(p2);

    vertices.iter().map(|v| (v.x, v.y)).collect()
}

const ROTATE_60: Vertex = Vertex {
    x: 1.0 / 2.0,
    // google の電卓を使用
    y: 1.73205080757 / 2.0,
};

// between p1 and p2 の頂点を全て配列に入れて返す
// current_depth -> 頂点を計算済みの n
// depth -> 最終的な目標の n
fn kock(start: &Vertex, end: &Vertex, current_depth: usize, depth: usize) -> Vec<Vertex> {
    let p1_to_s = &(end - start) * (1.0 / 3.0);

    let s = start + &p1_to_s;
    let p1_to_u = &p1_to_s + &(&p1_to_s * &ROTATE_60);
    let u = &s + &p1_to_u;
    let t = end - &p1_to_s;

    // 頂点の配列を初期化
    let mut vertices: Vec<Vertex> = Vec::new();

    // s
    vertices.push(s.clone());

    // between a and u
    if current_depth + 1 < depth {
        let s_and_u = kock(&s, &u, current_depth + 1, depth);
        vertices.extend(s_and_u);
    }

    // u
    vertices.push(u.clone());

    // between u and t
    if current_depth + 1 < depth {
        let u_and_t = kock(&u, &t, current_depth + 1, depth);
        vertices.extend(u_and_t);
    }

    // t
    vertices.push(t);

    vertices
}

#[derive(Clone)]
struct Vertex {
    x: f64,
    y: f64,
}

impl Add for &Vertex {
    type Output = Vertex;

    fn add(self, rhs: &Vertex) -> Self::Output {
        Vertex {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for &Vertex {
    type Output = Vertex;

    fn sub(self, rhs: &Vertex) -> Self::Output {
        Vertex {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Mul for &Vertex {
    type Output = Vertex;

    fn mul(self, rhs: &Vertex) -> Self::Output {
        Vertex {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }
}

impl Mul<f64> for &Vertex {
    type Output = Vertex;

    fn mul(self, rhs: f64) -> Self::Output {
        Vertex {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::solve;

    fn assert_points_close(actual: &[(f64, f64)], expected: &[(f64, f64)]) {
        const EPS: f64 = 1e-6;

        assert_eq!(actual.len(), expected.len());

        for (&(actual_x, actual_y), &(expected_x, expected_y)) in actual.iter().zip(expected) {
            assert!((actual_x - expected_x).abs() < EPS);
            assert!((actual_y - expected_y).abs() < EPS);
        }
    }

    #[test]
    fn sample_1() {
        let expected = vec![
            (0.0, 0.0),
            (33.33333333, 0.0),
            (50.0, 28.86751346),
            (66.66666667, 0.0),
            (100.0, 0.0),
        ];

        assert_points_close(&solve(1), &expected);
    }
}
