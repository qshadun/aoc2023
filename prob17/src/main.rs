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
    let layout = Layout::from_input(input, 1, 3);
    println!("part1 = {}", layout.find_min_loss());
}

fn part2(input: &str) {
    let layout = Layout::from_input(input, 4, 10);
    println!("part2 = {}", layout.find_min_loss());
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
struct Position {
    cor: Coordinate,
    dir: Direction,
}

impl Position {
    fn new(cor: Coordinate, dir: Direction) -> Self {
        Self { cor, dir }
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct PositionWithLoss {
    pos: Position,
    loss: usize,
}

impl PositionWithLoss {
    fn new(pos: Position, loss: usize) -> Self {
        Self { pos, loss }
    }
}

struct Layout {
    matrix: Vec<Vec<u8>>,
    min_step: usize,
    max_step: usize,
}

impl Layout {
    fn from_input(input: &str, min_step: usize, max_step: usize) -> Self {
        let mut matrix = vec![];
        for line in input.lines() {
            let row: Vec<u8> = line
                .chars()
                .map(|x| x.to_string().parse::<u8>().unwrap())
                .collect();
            matrix.push(row);
        }
        Self {
            matrix,
            min_step,
            max_step,
        }
    }

    fn find_min_loss(&self) -> usize {
        let start1 = Position::new(Coordinate::new(0, 0), Direction::Right);
        let start2 = Position::new(Coordinate::new(0, 0), Direction::Down);
        let mut q = VecDeque::new();
        q.push_back(PositionWithLoss::new(start1, 0));
        q.push_back(PositionWithLoss::new(start2, 0));
        let mut min_loss_map = HashMap::new();
        min_loss_map.insert(start1, 0);
        min_loss_map.insert(start2, 0);
        let m = self.matrix.len();
        let n = self.matrix[0].len();
        let end_cor = Coordinate::new(m - 1, n - 1);
        let max_step = (m + n) * 2;
        let mut min_loss = usize::MAX;
        let mut step = 0;
        while step < max_step && !q.is_empty() {
            let cur_len = q.len();
            for _ in 0..cur_len {
                let cur_pl = q.pop_front().unwrap();
                if cur_pl.loss > *min_loss_map.get(&cur_pl.pos).unwrap() {
                    continue;
                }
                for next_pl in self.next_positions(&cur_pl) {
                    let exist_loss = min_loss_map.entry(next_pl.pos).or_insert(usize::MAX);
                    if next_pl.loss < *exist_loss {
                        *exist_loss = next_pl.loss;
                        if next_pl.pos.cor == end_cor {
                            min_loss = min_loss.min(next_pl.loss);
                        }
                        q.push_back(next_pl);
                    }
                }
            }
            step += 1;
        }
        min_loss
    }

    fn next_positions(&self, cur: &PositionWithLoss) -> Vec<PositionWithLoss> {
        let mut ans = vec![];

        let turn_directions = match cur.pos.dir {
            Direction::Up | Direction::Down => [Direction::Left, Direction::Right],
            Direction::Left | Direction::Right => [Direction::Up, Direction::Down],
        };
        for dir in turn_directions {
            for pl in self.forward(cur.pos.cor, cur.loss, dir) {
                ans.push(pl);
            }
        }
        ans
    }

    fn forward(&self, pos: Coordinate, loss: usize, dir: Direction) -> Vec<PositionWithLoss> {
        let mut ans = vec![];
        let m = self.matrix.len();
        let n = self.matrix[0].len();
        let (x, y) = (pos.x, pos.y);
        match dir {
            Direction::Up => {
                if x >= self.min_step {
                    let mut extra_loss: usize = 0;
                    let mut nx = x;
                    for _ in 1..self.min_step {
                        nx -= 1;
                        extra_loss += self.matrix[nx][y] as usize;
                    }
                    for _ in self.min_step..=x.min(self.max_step) {
                        nx -= 1;
                        extra_loss += self.matrix[nx][y] as usize;
                        ans.push(PositionWithLoss::new(
                            Position::new(Coordinate::new(nx, y), dir),
                            loss + extra_loss,
                        ));
                    }
                }
            }
            Direction::Down => {
                if x + self.min_step <= m - 1 {
                    let mut extra_loss: usize = 0;
                    let mut nx = x;
                    for _ in 1..self.min_step {
                        nx += 1;
                        extra_loss += self.matrix[nx][y] as usize;
                    }
                    for _ in self.min_step..=(m - 1 - x).min(self.max_step) {
                        nx += 1;
                        extra_loss += self.matrix[nx][y] as usize;
                        ans.push(PositionWithLoss::new(
                            Position::new(Coordinate::new(nx, y), dir),
                            loss + extra_loss,
                        ));
                    }
                }
            }
            Direction::Left => {
                if y >= self.min_step {
                    let mut extra_loss: usize = 0;
                    let mut ny = y;
                    for _ in 1..self.min_step {
                        ny -= 1;
                        extra_loss += self.matrix[x][ny] as usize;
                    }
                    for _ in self.min_step..=y.min(self.max_step) {
                        ny -= 1;
                        extra_loss += self.matrix[x][ny] as usize;
                        ans.push(PositionWithLoss::new(
                            Position::new(Coordinate::new(x, ny), dir),
                            loss + extra_loss,
                        ));
                    }
                }
            }
            Direction::Right => {
                if y + self.min_step <= n - 1 {
                    let mut extra_loss: usize = 0;
                    let mut ny = y;
                    for _ in 1..self.min_step {
                        ny += 1;
                        extra_loss += self.matrix[x][ny] as usize;
                    }
                    for _ in self.min_step..=(n - 1 - y).min(self.max_step) {
                        ny += 1;
                        extra_loss += self.matrix[x][ny] as usize;
                        ans.push(PositionWithLoss::new(
                            Position::new(Coordinate::new(x, ny), dir),
                            loss + extra_loss,
                        ));
                    }
                }
            }
        }
        ans
    }
}
