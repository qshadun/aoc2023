use std::collections::HashSet;
use std::fs::read_to_string;

fn main() {
    let input = read_to_string("inputs/input10.txt").unwrap();
    let mut system = PipeSystem::from_input(&input);
    system.part1();
    system.part2();
}

struct PipeSystem {
    matrix: Vec<Vec<char>>,
}

impl PipeSystem {
    fn from_input(input: &str) -> Self {
        let mut matrix = vec![];
        for line in input.lines() {
            let row: Vec<char> = line.chars().collect();
            matrix.push(row);
        }
        Self { matrix }
    }

    fn find_start(&self) -> (usize, usize) {
        let mut start = (0, 0);
        'outer: for i in 0..self.matrix.len() {
            for j in 0..self.matrix[0].len() {
                if self.matrix[i][j] == 'S' {
                    start = (i, j);
                    break 'outer;
                }
            }
        }
        start
    }

    fn collected_cells(&self, r: usize, c: usize) -> Vec<(usize, usize)> {
        match self.matrix[r][c] {
            '|' => vec![(r - 1, c), (r + 1, c)],
            '-' => vec![(r, c - 1), (r, c + 1)],
            'L' => vec![(r - 1, c), (r, c + 1)],
            'J' => vec![(r - 1, c), (r, c - 1)],
            '7' => vec![(r + 1, c), (r, c - 1)],
            'F' => vec![(r + 1, c), (r, c + 1)],
            '.' => vec![],
            _ => panic!("invalid char"),
        }
    }

    fn part1(&self) {
        let start = self.find_start();

        let mut cur = vec![];
        for directions in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let x = (start.0 as i32 + directions.0) as usize;
            let y = (start.1 as i32 + directions.1) as usize;
            let connected = self.collected_cells(x, y);
            if connected.contains(&start) {
                cur.push((x, y));
            }
        }

        let mut prev = vec![start];
        let mut steps = 1;
        loop {
            let mut next = vec![];
            for (r, c) in &cur {
                for connected in self.collected_cells(*r, *c) {
                    if !prev.contains(&connected) {
                        next.push(connected);
                    }
                }
            }
            steps += 1;

            prev = cur;
            cur = next;
            if cur[0] == cur[1] {
                break;
            }
        }
        println!("part1 = {}", steps);
    }

    fn part2(&mut self) {
        let start = self.find_start();

        let cur = self.mark_start_and_find_step_1_pipes(start);

        let circle_pipes = self.find_circle(start, cur);

        let mut ans = 0;
        let mut inside = false;
        // scan from top to bottom and left to right, counting how many tiles are inside the loop.
        // keep track of a boolean that tells me if I'm inside the loop
        // every time I cross a vertical pipe that does not horizontally block the top (the place where I am in the loop), flip that state
        for i in 0..self.matrix.len() {
            for j in 0..self.matrix[0].len() {
                if circle_pipes.contains(&(i, j)) {
                    match self.matrix[i][j] {
                        '|' | 'J' | 'L' => inside = !inside,
                        _ => {}
                    }
                } else if inside {
                    ans += 1;
                }
            }
        }

        println!("part2 = {}", ans);
    }

    fn mark_start_and_find_step_1_pipes(&mut self, start: (usize, usize)) -> Vec<(usize, usize)> {
        let mut cur = vec![];
        let (x, y) = start;
        let (mut north, mut south, mut west, mut east) = (false, false, false, false);
        if self.collected_cells(x - 1, y).contains(&start) {
            north = true;
            cur.push((x - 1, y));
        }
        if self.collected_cells(x + 1, y).contains(&start) {
            south = true;
            cur.push((x + 1, y));
        }
        if self.collected_cells(x, y - 1).contains(&start) {
            west = true;
            cur.push((x, y - 1));
        }
        if self.collected_cells(x, y + 1).contains(&start) {
            east = true;
            cur.push((x, y + 1));
        }
        match (north, south, west, east) {
            (true, true, _, _) => self.matrix[x][y] = '|',
            (true, _, true, _) => self.matrix[x][y] = 'J',
            (true, _, _, true) => self.matrix[x][y] = 'L',
            (_, true, true, _) => self.matrix[x][y] = '7',
            (_, true, _, true) => self.matrix[x][y] = 'F',
            (_, _, true, true) => self.matrix[x][y] = '-',
            _ => panic!("illegal start"),
        }
        cur
    }

    fn find_circle(
        &self,
        start: (usize, usize),
        mut cur: Vec<(usize, usize)>,
    ) -> HashSet<(usize, usize)> {
        let mut prev = vec![start];
        let mut ans = HashSet::new();
        ans.insert(start);
        loop {
            let mut next = vec![];
            for (r, c) in &cur {
                for connected in self.collected_cells(*r, *c) {
                    if !prev.contains(&connected) {
                        next.push(connected);
                    }
                }
            }
            for p in &cur {
                ans.insert(*p);
            }
            prev = cur;
            cur = next;
            if cur[0] == cur[1] {
                ans.insert(cur[0]);
                break;
            }
        }
        ans
    }
}
