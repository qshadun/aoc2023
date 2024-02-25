use std::fs::read_to_string;

fn main() {
    let input = read_to_string("inputs/input12.txt").unwrap();
    part1(&input);
    part2(&input);
}

fn part1(input: &str) {
    let mut ans = 0;
    for line in input.lines() {
        let mut rec = Record::from_line(line, 1);
        ans += rec.calc_arrangements();
    }
    println!("part1 = {}", ans);
}

fn part2(input: &str) {
    let mut ans = 0;
    for line in input.lines() {
        let mut rec = Record::from_line(line, 5);
        ans += rec.calc_arrangements();
    }
    println!("part2 = {}", ans);
}

#[derive(Debug)]
struct Record {
    conditions: Vec<char>,
    groups: Vec<usize>,
    dp: Vec<Vec<i64>>,
}

impl Record {
    fn from_line(line: &str, fold: usize) -> Self {
        let parts: Vec<&str> = line.split(' ').collect();
        let mut conditions: Vec<char> = parts[0].chars().collect();
        let groups: Vec<usize> = parts[1].split(',').map(|x| x.parse().unwrap()).collect();
        if fold > 1 {
            let mut folded = conditions.clone();
            for _ in 0..fold - 1 {
                folded.push('?');
                folded.extend(&conditions);
            }
            conditions = folded;
        }

        let groups: Vec<usize> = groups
            .iter()
            .cycle()
            .take(groups.len() * fold)
            .copied()
            .collect();

        let m = conditions.len();
        let n = groups.len();
        let dp = vec![vec![-1; n + 1]; m];

        Self {
            conditions,
            groups,
            dp,
        }
    }

    fn calc_arrangements(&mut self) -> i64 {
        self.recur(0, 0)
    }

    fn recur(&mut self, i: usize, j: usize) -> i64 {
        let m = self.conditions.len();
        let n = self.groups.len();
        if i >= m && j >= n {
            return 1;
        }
        if i >= m {
            return 0;
        }
        if self.dp[i][j] != -1 {
            return self.dp[i][j];
        }
        if j == n {
            if self.conditions[i] == '#' {
                self.dp[i][j] = 0;
            } else {
                self.dp[i][j] = self.recur(i + 1, j);
            }
            return self.dp[i][j];
        }

        if i + self.groups[j] > m {
            self.dp[i][j] = 0;
            return 0;
        }
        if self.conditions[i] == '.' {
            self.dp[i][j] = self.recur(i + 1, j);
            return self.dp[i][j];
        }
        let mut ans = 0;
        if self.conditions[i] == '?' {
            ans += self.recur(i + 1, j);
        }
        let mut can_form = true;
        for k in i + 1..i + self.groups[j] {
            if self.conditions[k] == '.' {
                can_form = false;
                break;
            }
        }
        if can_form && i + self.groups[j] < m && self.conditions[i + self.groups[j]] == '#' {
            can_form = false;
        }
        if can_form {
            ans += self.recur(i + self.groups[j] + 1, j + 1);
        }

        self.dp[i][j] = ans;
        self.dp[i][j]
    }
}
