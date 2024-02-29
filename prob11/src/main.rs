use std::collections::HashSet;
use std::fs::read_to_string;

fn main() {
    let input = read_to_string("inputs/input11.txt").unwrap();
    let mut board = Board::from_input(&input);
    board.part1();

    let board = Board::from_input(&input);
    board.part2(1e6 as usize);
}

struct Board {
    matrix: Vec<Vec<char>>,
}

impl Board {
    fn from_input(input: &str) -> Self {
        let mut matrix = vec![];
        for line in input.lines() {
            let row: Vec<char> = line.chars().collect();
            matrix.push(row);
        }
        Self { matrix }
    }

    fn expand(&mut self) {
        self.expand_rows();
        self.expand_cols();
    }

    fn expand_rows(&mut self) {
        let mut expanded = vec![];
        for row in self.matrix.iter() {
            if row.contains(&'#') {
                expanded.push(row.clone());
            } else {
                expanded.push(row.clone());
                expanded.push(row.clone());
            }
        }
        self.matrix = expanded
    }

    fn expand_cols(&mut self) {
        let m = self.matrix.len();
        let n = self.matrix[0].len();
        let mut expanded = vec![vec![]; m];
        for j in 0..n {
            let mut has_galaxy = false;
            for i in 0..m {
                if self.matrix[i][j] == '#' {
                    has_galaxy = true;
                    break;
                }
            }
            for (i, e) in expanded.iter_mut().enumerate() {
                e.push(self.matrix[i][j]);
                if !has_galaxy {
                    e.push(self.matrix[i][j]);
                }
            }
        }
        self.matrix = expanded
    }

    fn part1(&mut self) {
        self.expand();
        let galaxies = self.collect_galaxies();
        let mut ans = 0;
        for i in 0..galaxies.len() - 1 {
            let (x1, y1) = galaxies[i];

            for (j, (x2, y2)) in galaxies.iter().enumerate().skip(i + 1) {
                if j == i {
                    continue;
                }
                ans += x1.abs_diff(*x2) + y1.abs_diff(*y2);
            }
        }
        println!("part1 = {}", ans);
    }

    fn collect_galaxies(&self) -> Vec<(i32, i32)> {
        let mut ans = vec![];
        for i in 0..self.matrix.len() {
            for j in 0..self.matrix[0].len() {
                if self.matrix[i][j] == '#' {
                    ans.push((i as i32, j as i32));
                }
            }
        }
        ans
    }

    fn rows_and_cols_need_expand(&self) -> (HashSet<usize>, HashSet<usize>) {
        let mut rows = HashSet::from_iter(0..self.matrix.len());
        let mut cols = HashSet::from_iter(0..self.matrix[0].len());
        for i in 0..self.matrix.len() {
            for j in 0..self.matrix[0].len() {
                if self.matrix[i][j] == '#' {
                    rows.remove(&i);
                    cols.remove(&j);
                }
            }
        }
        (rows, cols)
    }

    fn part2(&self, expand_scale: usize) {
        let mut ans = 0;
        let (rows, cols) = self.rows_and_cols_need_expand();
        let galaxies = self.collect_galaxies();
        for i in 0..galaxies.len() - 1 {
            let (x1, y1) = galaxies[i];
            let (x1, y1) = (x1 as usize, y1 as usize);
            for (j, (x2, y2)) in galaxies.iter().enumerate().skip(i + 1) {
                if j == i {
                    continue;
                }
                let (x2, y2) = (*x2 as usize, *y2 as usize);
                for x in x1.min(x2)..x1.max(x2) {
                    if rows.contains(&x) {
                        ans += expand_scale;
                    } else {
                        ans += 1;
                    }
                }
                for y in y1.min(y2)..y1.max(y2) {
                    if cols.contains(&y) {
                        ans += expand_scale;
                    } else {
                        ans += 1;
                    }
                }
            }
        }
        println!("part2 = {}", ans);
    }
}
