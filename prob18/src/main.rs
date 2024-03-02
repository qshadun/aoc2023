use std::fmt;
use std::fs::read_to_string;

fn main() {
    let input = read_to_string("inputs/input18.txt").unwrap();
    part1(&input);
    part2(&input);
}

fn part1(input: &str) {
    let maze = Maze::from_input(input);
    println!("part1 = {}", maze.count());
}

fn part2(input: &str) {
    let mut instructions = vec![];
    for line in input.lines() {
        instructions.push(parse_line2(line));
    }

    let mut area = 0;
    let mut preimeter = 0;
    let mut p = Point::new(0, 0);
    // showlace formula
    for (dir, steps) in instructions {
        let np = p.dig(dir, steps);
        area += p.x * np.y - np.x * p.y;
        preimeter += steps;
        p = np;
    }
    let ans = (area.abs() + preimeter) / 2 + 1;
    println!("part2 = {}", ans);
}

#[derive(Debug, Clone, Copy)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }

    fn dig(&self, direction: char, steps: i64) -> Self {
        match direction {
            'L' => Self::new(self.x, self.y - steps),
            'R' => Self::new(self.x, self.y + steps),
            'U' => Self::new(self.x - steps, self.y),
            'D' => Self::new(self.x + steps, self.y),
            _ => panic!("invalid char"),
        }
    }
}

struct Maze {
    matrix: Vec<Vec<char>>,
}

impl Maze {
    fn from_input(input: &str) -> Self {
        let (mut min_x, mut min_y, mut max_x, mut max_y) = (0, 0, 0, 0);
        let (mut x, mut y) = (0, 0);
        for line in input.lines() {
            let (d, steps) = Self::parse_line(line);

            match d {
                'R' => {
                    y += steps;
                    max_y = max_y.max(y);
                }
                'L' => {
                    y -= steps;
                    min_y = min_y.min(y);
                }
                'D' => {
                    x += steps;
                    max_x = max_x.max(x);
                }
                'U' => {
                    x -= steps;
                    min_x = min_x.min(x);
                }
                _ => panic!("wrong direction"),
            }
        }
        let m = (max_x - min_x) as usize + 1;
        let n = (max_y - min_y) as usize + 1;
        let mut matrix = vec![vec!['.'; n]; m];

        let (mut x, mut y) = ((0 - min_x) as usize, (0 - min_y) as usize);
        matrix[x][y] = '#';
        for line in input.lines() {
            let (d, steps) = Self::parse_line(line);
            match d {
                'R' => {
                    for _ in 1..=steps {
                        y += 1;
                        matrix[x][y] = '#';
                    }
                }
                'L' => {
                    for _ in 1..=steps {
                        y -= 1;
                        matrix[x][y] = '#';
                    }
                }
                'D' => {
                    for _ in 1..=steps {
                        x += 1;
                        matrix[x][y] = '#';
                    }
                }
                'U' => {
                    for _ in 1..=steps {
                        x -= 1;
                        matrix[x][y] = '#';
                    }
                }
                _ => panic!("wrong direction"),
            }
        }
        Self { matrix }
    }

    fn parse_line(line: &str) -> (char, i32) {
        let parts: Vec<_> = line.split(' ').collect();
        let d = parts[0].chars().next().unwrap();
        let steps: i32 = parts[1].parse().unwrap();
        (d, steps)
    }

    fn count(&self) -> usize {
        let mut ans = 0;

        for i in 0..self.matrix.len() {
            let mut inside = false;
            for j in 0..self.matrix[0].len() {
                match self.matrix[i][j] {
                    '#' => {
                        ans += 1;
                        if i > 0 && self.matrix[i - 1][j] == '#' {
                            inside = !inside;
                        }
                    }
                    '.' => {
                        if inside {
                            ans += 1;
                        }
                    }
                    _ => panic!("invalid char"),
                }
            }
        }
        ans
    }
}

impl fmt::Debug for Maze {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in self.matrix.iter() {
            _ = writeln!(f, "{}", row.iter().collect::<String>());
        }
        Ok(())
    }
}

fn parse_line2(line: &str) -> (char, i64) {
    //R 3 (#63d832)
    let parts: Vec<_> = line.split(' ').collect();
    let s = &parts[2][2..8];
    let steps = i64::from_str_radix(&s[0..5], 16).unwrap();
    let d = match s.chars().nth(5).unwrap() {
        '0' => 'R',
        '1' => 'D',
        '2' => 'L',
        '3' => 'U',
        _ => panic!("invalid char"),
    };
    (d, steps)
}
