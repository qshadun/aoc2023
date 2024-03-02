use std::{
    collections::{HashMap, VecDeque},
    fs::read_to_string,
};

fn main() {
    let input = read_to_string("inputs/input17.txt").unwrap();
    part1(&input);
    part2(&input);
}

fn part1(input: &str) {
    let layout = Layout::from_input(input);
    println!("part1 = {}", layout.find_min_loss());
}

fn part2(input: &str) {}

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
struct Position {
    cor: Coordinate,
    dir: Direction,
    streak: u8,
}

impl Position {
    fn new(cor: Coordinate, dir: Direction, streak: u8) -> Self {
        Self { cor, dir, streak }
    }
}

struct Layout {
    matrix: Vec<Vec<u8>>,
}

impl Layout {
    fn from_input(input: &str) -> Self {
        let mut matrix = vec![];
        for line in input.lines() {
            let row: Vec<u8> = line
                .chars()
                .map(|x| x.to_string().parse::<u8>().unwrap())
                .collect();
            matrix.push(row);
        }
        Self { matrix }
    }

    fn find_min_loss(&self) -> usize {
        let start = Position::new(Coordinate::new(0, 0), Direction::Right, 1);
        let mut q = VecDeque::new();
        q.push_back((start, 0));
        let mut min_loss_map = HashMap::new();
        min_loss_map.insert(start, 0);
        let m = self.matrix.len();
        let n = self.matrix[0].len();
        let end_cor = Coordinate::new(m - 1, n - 1);
        let max_step = (m + n) * 2;
        let mut min_loss = usize::MAX;
        let mut step = 0;
        while step < max_step && !q.is_empty() {
            let cur_len = q.len();
            for _ in 0..cur_len {
                let (cur_pos, cur_loss) = q.pop_front().unwrap();
                if cur_loss > *min_loss_map.get(&cur_pos).unwrap() {
                    continue;
                }
                for next_pos in self.next_positions(&cur_pos) {
                    let loss = self.get_loss(next_pos.cor) as usize + cur_loss;

                    let exist_loss = min_loss_map.entry(next_pos).or_insert(usize::MAX);
                    if loss < *exist_loss {
                        *exist_loss = loss;
                        q.push_back((next_pos, loss));
                        if next_pos.cor == end_cor {
                            min_loss = min_loss.min(loss);
                        }
                    }
                }
            }
            step += 1;
        }
        min_loss
    }

    fn next_positions(&self, cur: &Position) -> Vec<Position> {
        let mut ans = vec![];
        if cur.streak < 3 {
            if let Some(next_cor) = self.forward(&cur.cor, cur.dir) {
                ans.push(Position::new(next_cor, cur.dir, cur.streak + 1));
            }
        }
        let turn_directions = match cur.dir {
            Direction::Up | Direction::Down => [Direction::Left, Direction::Right],
            Direction::Left | Direction::Right => [Direction::Up, Direction::Down],
        };
        for dir in turn_directions {
            if let Some(next_cor) = self.forward(&cur.cor, dir) {
                ans.push(Position::new(next_cor, dir, 1));
            }
        }
        ans
    }

    fn forward(&self, pos: &Coordinate, dir: Direction) -> Option<Coordinate> {
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

    fn get_loss(&self, cor: Coordinate) -> u8 {
        self.matrix[cor.x][cor.y]
    }
}
