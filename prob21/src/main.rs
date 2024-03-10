use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs::read_to_string,
};

use utils::Point;

fn main() {
    let input = read_to_string("inputs/input21.txt").unwrap();
    part1(&input);
    // part2(&input);
    parity(&input);
}

fn part1(input: &str) {
    let map = Map::from_input(input);
    let ans = map.possible_positions(64);
    println!("part1 = {ans}");
}

#[allow(dead_code)]
fn part2(input: &str) {
    let map = Map::from_input(input);
    let pp = map.possible_positions_infility(50);
    for (i, v) in pp.iter().enumerate() {
        println!("{i} {v}");
    }
    println!("*************");
    for i in 1..pp.len() {
        println!("{i} {:?}", pp[i] - pp[i - 1]);
    }
}

fn parity(input: &str) {
    // https://github.com/villuna/aoc23/wiki/A-Geometric-solution-to-advent-of-code-2023,-day-21
    let map = Map::from_input(input);
    println!("dimension is {} {}", map.m, map.n);
    let counter = map.count_steps();
    let part1 = counter
        .values()
        .filter(|x| **x <= 64 && **x % 2 == 0)
        .count();
    println!("part1 = {part1}");
    // counter is a HashMap<Point, usize> which maps tiles in the input-square to their distance from the starting tile
    // So read this as "even_corners is the number of tiles which have a distance that is even and greater than 65"
    let even_corners = counter
        .values()
        .filter(|v| **v % 2 == 0 && **v > 65)
        .count();
    let odd_corners = counter
        .values()
        .filter(|v| **v % 2 == 1 && **v > 65)
        .count();

    let even_full = counter.values().filter(|v| **v % 2 == 0).count();
    let odd_full = counter.values().filter(|v| **v % 2 == 1).count();

    // This is 202300 but im writing it out here to show the process
    let n = (26501365 - (map.m / 2)) / map.m;
    assert_eq!(n, 202300);

    let p2 = ((n + 1) * (n + 1)) * odd_full + (n * n) * even_full - (n + 1) * odd_corners
        + n * even_corners;
    println!("part2 = {p2}");
}

#[derive(Debug, Clone)]
struct Map {
    matrix: Vec<Vec<char>>,
    m: usize,
    n: usize,
    start: Point,
}

#[allow(dead_code)]
impl Map {
    fn from_input(input: &str) -> Self {
        let mut start = Point::new(0, 0);
        let mut matrix = vec![];
        for line in input.lines() {
            let row: Vec<char> = line.chars().collect();
            matrix.push(row);
        }
        let m = matrix.len();
        let n = matrix[0].len();
        'out: for (i, row) in matrix.iter().enumerate() {
            for (j, c) in row.iter().enumerate() {
                if *c == 'S' {
                    start = Point::new(i, j);
                    break 'out;
                }
            }
        }
        Self {
            matrix,
            m,
            n,
            start,
        }
    }

    fn possible_positions(&self, steps: usize) -> usize {
        let mut cur_positions = HashSet::new();
        cur_positions.insert(self.start);
        for _ in 0..steps {
            let mut new_positions = HashSet::new();
            for p in cur_positions.into_iter() {
                let moves = p.moves(self.m, self.n);
                new_positions.extend(
                    moves
                        .into_iter()
                        .filter(|Point { x, y }| self.matrix[*x][*y] != '#'),
                );
            }
            cur_positions = new_positions;
        }
        cur_positions.len()
    }

    fn is_valid(&self, x: i32, y: i32) -> bool {
        let x = Self::to_index(x, self.m);
        let y = Self::to_index(y, self.n);
        self.matrix[x][y] != '#'
    }

    fn to_index(i: i32, len: usize) -> usize {
        let mut ans = i % len as i32;
        if ans < 0 {
            ans += len as i32;
        }
        ans as usize
    }

    fn valid_moves(&self, p: InfinityPoint) -> Vec<InfinityPoint> {
        let mut ans = vec![];
        let (i, j) = (p.x, p.y);
        for (x, y) in [(i - 1, j), (i + 1, j), (i, j - 1), (i, j + 1)] {
            if self.is_valid(x, y) {
                ans.push(InfinityPoint::new(x, y));
            }
        }
        ans
    }

    fn possible_positions_infility(&self, steps: usize) -> Vec<usize> {
        let mut ans = vec![1];
        let mut cur_positions = HashSet::new();

        cur_positions.insert(InfinityPoint::from_point(self.start));
        for _ in 0..steps {
            let mut new_positions = HashSet::new();
            for p in cur_positions.into_iter() {
                new_positions.extend(self.valid_moves(p));
            }
            cur_positions = new_positions;
            ans.push(cur_positions.len());
        }
        ans
    }

    fn count_steps(&self) -> HashMap<Point, i32> {
        let mut visited = HashMap::new();
        visited.insert(self.start, 0);
        let mut q = VecDeque::new();
        q.push_back((self.start, 0));
        while let Some((p, steps)) = q.pop_front() {
            let moves = p
                .moves(self.m, self.n)
                .into_iter()
                .filter(|x| self.matrix[x.x][x.y] != '#');
            for pp in moves {
                visited.entry(pp).or_insert_with(|| {
                    q.push_back((pp, steps + 1));
                    steps + 1
                });
            }
        }
        visited
    }
}

#[derive(Debug, Clone, Hash, PartialEq, PartialOrd, Ord, Eq)]
struct InfinityPoint {
    x: i32,
    y: i32,
}

impl InfinityPoint {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn from_point(p: Point) -> Self {
        Self::new(p.x as i32, p.y as i32)
    }
}
