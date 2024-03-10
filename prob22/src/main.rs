use std::{
    collections::{HashSet, VecDeque},
    fs::read_to_string,
};

fn main() {
    let input = read_to_string("inputs/input22.txt").unwrap();
    let game = Game::from_input(&input);
    let (p1, p2) = game.solve();
    println!("part1 = {p1}");
    println!("part2 = {p2}");
}

#[derive(Debug, Clone, Copy)]
struct Brick {
    x0: i32,
    x1: i32,
    y0: i32,
    y1: i32,
    z1: i32,
    z2: i32,
}

impl Brick {
    fn from_line(line: &str) -> Self {
        let parts: Vec<_> = line.split('~').collect();
        let (x1, y1, z1) = Self::parse_cords(parts[0]);
        let (x2, y2, z2) = Self::parse_cords(parts[1]);
        Self {
            x0: x1.min(x2),
            x1: x1.max(x2),
            y0: y1.min(y2),
            y1: y1.max(y2),
            z1: z1.min(z2),
            z2: z1.max(z2),
        }
    }

    fn parse_cords(s: &str) -> (i32, i32, i32) {
        let parts: Vec<_> = s.split(',').collect();
        (
            parts[0].parse().unwrap(),
            parts[1].parse().unwrap(),
            parts[2].parse().unwrap(),
        )
    }

    fn is_intersect(&self, other: &Brick) -> bool {
        self.x0 <= other.x1 && self.x1 >= other.x0 && self.y0 <= other.y1 && self.y1 >= other.y0
    }
}

#[derive(Debug)]
struct Game {
    bricks: Vec<Brick>,
}

impl Game {
    fn from_input(input: &str) -> Self {
        let mut bricks = vec![];
        for line in input.lines() {
            bricks.push(Brick::from_line(line));
        }
        bricks.sort_by_key(|b| b.z1);
        Self { bricks }
    }

    fn solve(&self) -> (usize, usize) {
        let n = self.bricks.len();

        let mut above: Vec<Vec<usize>> = vec![vec![]; n];
        let mut below: Vec<Vec<usize>> = vec![vec![]; n];

        // fall bricks
        let mut fall_down: Vec<Vec<usize>> = vec![vec![]; 1];
        for (i, b) in self.bricks.iter().enumerate() {
            let mut support_level = 0;
            for (cur_level, cur_level_bricks) in fall_down.iter().enumerate().rev() {
                for j in cur_level_bricks {
                    if b.is_intersect(&self.bricks[*j]) {
                        support_level = cur_level;
                        below[i].push(*j);
                        above[*j].push(i);
                    }
                }
                if support_level > 0 {
                    break;
                }
            }
            let height = (b.z2 - b.z1) as usize;
            if support_level == fall_down.len() - 1 {
                for _ in 0..height {
                    fall_down.push(vec![]);
                }
                fall_down.push(vec![i]);
            } else {
                for _ in fall_down.len()..=(support_level + 1 + height) {
                    fall_down.push(vec![]);
                }
                fall_down[support_level + 1 + height].push(i);
            }
        }

        let mut p1 = 0;
        for x in above.iter() {
            if x.iter().all(|j| below[*j].len() > 1) {
                p1 += 1;
            }
        }
        let mut p2 = 0;
        for i in 0..n {
            let mut q = VecDeque::new();
            q.push_back(i);
            let mut removed = HashSet::new();
            removed.insert(i);
            while let Some(cur) = q.pop_front() {
                for j in above[cur].iter() {
                    if !removed.contains(j) && below[*j].iter().all(|x| removed.contains(x)) {
                        removed.insert(*j);
                        q.push_back(*j);
                    }
                }
            }
            p2 += removed.len() - 1;
        }

        (p1, p2)
    }
}
