use std::{
    collections::HashMap,
    fmt::{Debug, Write},
    fs::read_to_string,
};

fn main() {
    let input = read_to_string("inputs/input14.txt").unwrap();
    let mut board = Board::from_input(&input);
    board.tilt_north();
    println!("part1 = {}", board.calc_load());

    let mut board = Board::from_input(&input);
    let part2 = board.cycle(1000000000);
    println!("part2 = {:?}", part2);
}

struct Board {
    matrix: Vec<Vec<char>>,
}

impl Debug for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.matrix.iter() {
            let s: String = row.iter().collect();
            f.write_str(&s)?;
            f.write_char('\n')?;
        }
        Ok(())
    }
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

    fn tilt_north(&mut self) {
        let m = self.matrix.len();
        let n = self.matrix[0].len();
        for i in 0..m {
            for j in 0..n {
                if self.matrix[i][j] == 'O' {
                    let mut k = i as i32 - 1;
                    while k >= 0 {
                        if self.matrix[k as usize][j] == '.' {
                            k -= 1;
                        } else {
                            break;
                        }
                    }
                    self.matrix[i][j] = '.';
                    self.matrix[(k + 1) as usize][j] = 'O';
                }
            }
        }
    }

    fn tilt_west(&mut self) {
        let m = self.matrix.len();
        let n = self.matrix[0].len();
        for i in 0..m {
            for j in 0..n {
                if self.matrix[i][j] == 'O' {
                    let mut k = j as i32 - 1;
                    while k >= 0 {
                        if self.matrix[i][k as usize] == '.' {
                            k -= 1;
                        } else {
                            break;
                        }
                    }
                    self.matrix[i][j] = '.';
                    self.matrix[i][(k + 1) as usize] = 'O';
                }
            }
        }
    }

    fn tilt_south(&mut self) {
        let m = self.matrix.len();
        let n = self.matrix[0].len();
        for i in (0..m).rev() {
            for j in 0..n {
                if self.matrix[i][j] == 'O' {
                    let mut k = i + 1;
                    while k < m {
                        if self.matrix[k][j] == '.' {
                            k += 1;
                        } else {
                            break;
                        }
                    }
                    self.matrix[i][j] = '.';
                    self.matrix[k - 1][j] = 'O';
                }
            }
        }
    }

    fn tilt_east(&mut self) {
        let m = self.matrix.len();
        let n = self.matrix[0].len();
        for i in 0..m {
            for j in (0..n).rev() {
                if self.matrix[i][j] == 'O' {
                    let mut k = j + 1;
                    while k < n {
                        if self.matrix[i][k] == '.' {
                            k += 1;
                        } else {
                            break;
                        }
                    }
                    self.matrix[i][j] = '.';
                    self.matrix[i][k - 1] = 'O';
                }
            }
        }
    }

    fn calc_load(&self) -> usize {
        let mut ans = 0;
        let m = self.matrix.len();
        let n = self.matrix[0].len();
        for i in 0..m {
            for j in 0..n {
                if self.matrix[i][j] == 'O' {
                    ans += m - i;
                }
            }
        }
        ans
    }

    fn cycle(&mut self, n: usize) -> usize {
        let mut scores = vec![];
        let mut patterns = HashMap::new();
        scores.push(self.calc_load());
        for i in 0..n {
            self.tilt_north();
            self.tilt_west();
            self.tilt_south();
            self.tilt_east();
            scores.push(self.calc_load());
            if i >= 7 {
                let pattern = (
                    scores[i - 6],
                    scores[i - 5],
                    scores[i - 4],
                    scores[i - 3],
                    scores[i - 2],
                    scores[i - 1],
                    scores[i],
                );
                if let std::collections::hash_map::Entry::Vacant(e) = patterns.entry(pattern) {
                    e.insert(i - 6);
                } else {
                    let start = patterns.get(&pattern).unwrap();
                    let pattern_len = i - 6 - start;
                    println!("{} {}", start, pattern_len);

                    return scores[start + (n - start) % pattern_len];
                }
            }
        }
        0
    }
}
