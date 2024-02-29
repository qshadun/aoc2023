use std::{
    collections::{HashSet, VecDeque},
    fs::read_to_string,
};

fn main() {
    let input = read_to_string("inputs/input16.txt").unwrap();
    part1(&input);
    part2(&input);
}

fn part1(input: &str) {
    let layout = Layout::from_input(input);
    let start = Beam::new(Coordinate::new(0, 0), Direction::Right);
    println!("part1 = {}", layout.calc_energized(start));
}

fn part2(input: &str) {
    let mut ans = 0;
    let layout = Layout::from_input(input);
    let m = layout.matrix.len();
    let n = layout.matrix[0].len();
    for i in 0..m {
        let e = layout.calc_energized(Beam::new(Coordinate::new(i, 0), Direction::Right));
        ans = ans.max(e);
        let e = layout.calc_energized(Beam::new(Coordinate::new(i, n - 1), Direction::Left));
        ans = ans.max(e);
    }

    for j in 0..n {
        let e = layout.calc_energized(Beam::new(Coordinate::new(0, j), Direction::Down));
        ans = ans.max(e);
        let e = layout.calc_energized(Beam::new(Coordinate::new(m - 1, j), Direction::Up));
        ans = ans.max(e);
    }
    println!("part2 = {}", ans);
}

#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq)]
struct Coordinate {
    x: usize,
    y: usize,
}

impl Coordinate {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Beam {
    cor: Coordinate,
    dir: Direction,
}

impl Beam {
    fn new(cor: Coordinate, dir: Direction) -> Self {
        Self { cor, dir }
    }
}

struct Layout {
    matrix: Vec<Vec<char>>,
}

impl Layout {
    fn from_input(input: &str) -> Self {
        let mut matrix = vec![];
        for line in input.lines() {
            let row: Vec<char> = line.chars().collect();
            matrix.push(row);
        }
        Self { matrix }
    }

    fn travel(&self, beam: &Beam) -> Vec<Beam> {
        let mut ans = vec![];
        match self.get_char(beam.cor) {
            '\\' => match beam.dir {
                Direction::Up => {
                    if let Some(np) = self.next_pos(beam.cor, Direction::Left) {
                        ans.push(Beam::new(np, Direction::Left));
                    }
                }
                Direction::Down => {
                    if let Some(np) = self.next_pos(beam.cor, Direction::Right) {
                        ans.push(Beam::new(np, Direction::Right));
                    }
                }
                Direction::Left => {
                    if let Some(np) = self.next_pos(beam.cor, Direction::Up) {
                        ans.push(Beam::new(np, Direction::Up));
                    }
                }
                Direction::Right => {
                    if let Some(np) = self.next_pos(beam.cor, Direction::Down) {
                        ans.push(Beam::new(np, Direction::Down));
                    }
                }
            },
            '/' => match beam.dir {
                Direction::Up => {
                    if let Some(np) = self.next_pos(beam.cor, Direction::Right) {
                        ans.push(Beam::new(np, Direction::Right));
                    }
                }
                Direction::Down => {
                    if let Some(np) = self.next_pos(beam.cor, Direction::Left) {
                        ans.push(Beam::new(np, Direction::Left));
                    }
                }
                Direction::Left => {
                    if let Some(np) = self.next_pos(beam.cor, Direction::Down) {
                        ans.push(Beam::new(np, Direction::Down));
                    }
                }
                Direction::Right => {
                    if let Some(np) = self.next_pos(beam.cor, Direction::Up) {
                        ans.push(Beam::new(np, Direction::Up));
                    }
                }
            },
            '|' => match beam.dir {
                Direction::Up | Direction::Down => {
                    if let Some(next_pos) = self.next_pos(beam.cor, beam.dir) {
                        ans.push(Beam::new(next_pos, beam.dir));
                    }
                }

                Direction::Left | Direction::Right => {
                    if let Some(np) = self.next_pos(beam.cor, Direction::Up) {
                        ans.push(Beam::new(np, Direction::Up));
                    }
                    if let Some(np) = self.next_pos(beam.cor, Direction::Down) {
                        ans.push(Beam::new(np, Direction::Down));
                    }
                }
            },
            '-' => match beam.dir {
                Direction::Up | Direction::Down => {
                    if let Some(np) = self.next_pos(beam.cor, Direction::Left) {
                        ans.push(Beam::new(np, Direction::Left));
                    }
                    if let Some(np) = self.next_pos(beam.cor, Direction::Right) {
                        ans.push(Beam::new(np, Direction::Right));
                    }
                }

                Direction::Left | Direction::Right => {
                    if let Some(next_pos) = self.next_pos(beam.cor, beam.dir) {
                        ans.push(Beam::new(next_pos, beam.dir));
                    }
                }
            },
            _ => {
                if let Some(next_pos) = self.next_pos(beam.cor, beam.dir) {
                    ans.push(Beam::new(next_pos, beam.dir));
                }
            }
        }
        ans
    }

    fn get_char(&self, cor: Coordinate) -> char {
        self.matrix[cor.x][cor.y]
    }

    fn next_pos(&self, pos: Coordinate, dir: Direction) -> Option<Coordinate> {
        let (x, y) = (pos.x, pos.y);
        match dir {
            Direction::Up => {
                if x > 0 {
                    Some(Coordinate { x: x - 1, y })
                } else {
                    None
                }
            }
            Direction::Down => {
                if x < self.matrix.len() - 1 {
                    Some(Coordinate { x: x + 1, y })
                } else {
                    None
                }
            }
            Direction::Left => {
                if y > 0 {
                    Some(Coordinate { x, y: y - 1 })
                } else {
                    None
                }
            }
            Direction::Right => {
                if y < self.matrix[0].len() - 1 {
                    Some(Coordinate { x, y: y + 1 })
                } else {
                    None
                }
            }
        }
    }

    fn calc_energized(&self, start: Beam) -> usize {
        let mut q = VecDeque::new();
        q.push_back(start);
        let mut visited = HashSet::new();
        visited.insert(start);
        while let Some(b) = q.pop_front() {
            for bb in self.travel(&b) {
                if !visited.contains(&bb) {
                    q.push_back(bb);
                    visited.insert(bb);
                }
            }
        }
        let energized: HashSet<_> = visited.into_iter().map(|b| b.cor).collect();
        energized.len()
    }
}
