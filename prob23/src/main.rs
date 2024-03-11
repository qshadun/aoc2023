use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs::read_to_string,
};
use utils::Point;

fn main() {
    let input = read_to_string("inputs/input23.txt").unwrap();
    part1(&input);
    part2(&input);
}

fn part1(input: &str) {
    let mut game = Game::from_input(input);
    game.backtrack(game.start, 0);
    println!("part1 = {}", game.max_steps);
}

fn part2(input: &str) {
    let mut game = Game::from_input(input);
    game.replace_slopes();
    let vertexes = game.find_vertexes();
    let edges = game.find_edges(&vertexes);
    let mut seen = HashSet::new();
    game.backtrack_vertex(&edges, &mut seen, game.start, 0);
    println!("part2 = {}", game.max_steps);
}

#[derive(Debug)]
struct Game {
    matrix: Vec<Vec<char>>,
    m: usize,
    n: usize,
    start: Point,
    end: Point,
    max_steps: usize,
}

impl Game {
    fn from_input(input: &str) -> Self {
        let mut matrix = vec![];
        for line in input.lines() {
            let row: Vec<char> = line.chars().collect();
            matrix.push(row);
        }
        let i = matrix[0].iter().position(|x| *x == '.').unwrap();
        let start = Point::new(0, i);
        let m = matrix.len();
        let n = matrix[0].len();
        let i = matrix[m - 1].iter().position(|x| *x == '.').unwrap();
        let end = Point::new(m - 1, i);
        Self {
            matrix,
            m,
            n,
            start,
            end,
            max_steps: 0,
        }
    }

    fn backtrack(&mut self, cur: Point, steps: usize) {
        if cur == self.end {
            self.max_steps = self.max_steps.max(steps);
            return;
        }
        match self.matrix[cur.x][cur.y] {
            '.' => {
                self.matrix[cur.x][cur.y] = 'O';
                for np in self.valid_moves(cur) {
                    self.backtrack(np, steps + 1);
                }
                self.matrix[cur.x][cur.y] = '.';
            }
            '>' => {
                let np = Point::new(cur.x, cur.y + 1);
                if self.is_valid_move(&np) {
                    self.backtrack(np, steps + 1);
                }
            }
            '<' => {
                let np = Point::new(cur.x, cur.y - 1);
                if self.is_valid_move(&np) {
                    self.backtrack(np, steps + 1);
                }
            }
            '^' => {
                let np = Point::new(cur.x - 1, cur.y);
                if self.is_valid_move(&np) {
                    self.backtrack(np, steps + 1);
                }
            }
            'v' => {
                let np = Point::new(cur.x + 1, cur.y);
                if self.is_valid_move(&np) {
                    self.backtrack(np, steps + 1);
                }
            }
            x => panic!("invalid pos {:?} {}", cur, x),
        }
    }

    fn valid_moves(&self, cur: Point) -> Vec<Point> {
        let moves = cur.moves(self.m, self.n);
        moves
            .into_iter()
            .filter(|x| self.is_valid_move(x))
            .collect()
    }

    fn is_valid_move(&self, p: &Point) -> bool {
        self.matrix[p.x][p.y] != '#' && self.matrix[p.x][p.y] != 'O'
    }

    fn replace_slopes(&mut self) {
        for i in 0..self.m {
            for j in 0..self.n {
                if self.matrix[i][j] != '#' && self.matrix[i][j] != '.' {
                    self.matrix[i][j] = '.';
                }
            }
        }
    }

    // https://github.com/derailed-dash/Advent-of-Code/blob/master/src/AoC_2023/Dazbo's_Advent_of_Code_2023.ipynb
    //We need to represent the grid as a graph. Recall that a graph is a model that represents a set of vertices, linked together by edges.
    // We can use edge contraction to eliminate all the nodes where there is no path choice. I.e. for nodes where the number of edges is two, then we can eliminate these nodes from the graph. Why? Because if the number of edges is two, then the node only has a single forward and a single backward path. This helps improve the performance of our longest path solution.
    // Thus, vertices that we want to keep will have at least three viable next moves.
    // How to do this?
    // For each point in the grid (that isn't a wall), identify how many valid neighbours a point has, where a neighbour is a valid next move. If we're in a channel, then there will only be two neighbours: forwards and backwards. But if we're at a vertex, then there will be more than two. Store these vertices.
    // Add our start and end points to the set of vertices.
    // Next, walk the maze from each vertex to the next, using BFS. Whenever we detect the next vertex, join it to the previous vertex to create an edge. The edge weight is the distance between one vertex and the next.
    //
    fn find_vertexes(&self) -> HashSet<Point> {
        let mut ans = HashSet::new();
        for (i, row) in self.matrix.iter().enumerate() {
            for (j, c) in row.iter().enumerate() {
                if *c == '.' && self.valid_moves(Point::new(i, j)).len() > 2 {
                    ans.insert(Point::new(i, j));
                }
            }
        }
        ans.insert(self.start);
        ans.insert(self.end);
        ans
    }

    fn find_edges(&self, vertexes: &HashSet<Point>) -> HashMap<Point, Vec<(Point, usize)>> {
        let mut ans = HashMap::new();
        for v in vertexes {
            let mut neighbors = vec![];
            let mut visited = HashSet::new();
            visited.insert(*v);
            let mut q = VecDeque::new();
            q.push_back((*v, 0));
            while let Some((p, dist)) = q.pop_front() {
                for np in self.valid_moves(p) {
                    if !visited.contains(&np) {
                        visited.insert(np);
                        if vertexes.contains(&np) {
                            neighbors.push((np, dist + 1));
                        } else {
                            q.push_back((np, dist + 1));
                        }
                    }
                }
            }
            ans.insert(*v, neighbors);
        }
        ans
    }

    fn backtrack_vertex(
        &mut self,
        edges: &HashMap<Point, Vec<(Point, usize)>>,
        seen: &mut HashSet<Point>,
        cur: Point,
        dist: usize,
    ) {
        if cur == self.end {
            self.max_steps = self.max_steps.max(dist);
            return;
        }
        seen.insert(cur);
        for (np, nd) in edges.get(&cur).unwrap() {
            if !seen.contains(np) {
                self.backtrack_vertex(edges, seen, *np, dist + nd);
            }
        }
        seen.remove(&cur);
    }
}
