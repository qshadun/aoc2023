use ndarray::prelude::*;
use ndarray_linalg::Solve;

fn main() {
    println!("Hello, world!");
}

#[derive(Debug, Clone, Copy)]
struct Hail {
    px: f64,
    py: f64,
    pz: f64,
    vx: f64,
    vy: f64,
    vz: f64,
}

impl Hail {
    fn from_line(line: &str) -> Self {
        let parts: Vec<_> = line.split(" @ ").collect();
        let (px, py, pz) = Self::parse_numbers(parts[0]);
        let (vx, vy, vz) = Self::parse_numbers(parts[1]);
        Self {px, py, pz, vx, vy, vz}
    }

    fn parse_numbers(s: &str) -> (f64, f64, f64) {
        let parts: Vec<_> = s.split(", ").collect();
        let x = parts[0].trim().parse().unwrap();
        let y = parts[1].trim().parse().unwrap();
        let z = parts[2].trim().parse().unwrap();
        (x, y, z)
    }

    // px1 - vx1 * t1 = px2 - vx2 * t2
    // py1 - vy1 * t1 = py2 - vy2 * t2
    fn intersect(&self, other: &Hail) -> Option<(f64, f64)> {
        let a: Array2<f64> = array![[self.vx, other.vx], [self.vy, other.vy]];
        let b: Array1<f64>  = array![self.px-other.px, self.py-other.py];
        match a.solve_into(b) {
            Ok(x) => {
                let t = x[0];
                Some((self.px + self.vx * t, self.py + self.vy * t))
            }
            Err(_) => {
                None
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let h1 = Hail::from_line("19, 13, 30 @ -2,  1, -2");
        let h2 = Hail::from_line("18, 19, 22 @ -1, -1, -2");
        println!("{:?}", h1.intersect(&h2));
    }
}