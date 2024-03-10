#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Ord, Eq, Hash)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

impl Point {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    pub fn moves(&self, m: usize, n: usize) -> Vec<Self> {
        let mut ans = vec![];
        if self.x > 0 {
            ans.push(Self::new(self.x - 1, self.y));
        }
        if self.x < m - 1 {
            ans.push(Self::new(self.x + 1, self.y));
        }
        if self.y > 0 {
            ans.push(Self::new(self.x, self.y - 1));
        }
        if self.y < n - 1 {
            ans.push(Self::new(self.x, self.y + 1));
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn moves() {
        let p = Point::new(0, 0);
        assert_eq!(p.moves(10, 10), vec![Point::new(1, 0), Point::new(0, 1)]);
    }
}
