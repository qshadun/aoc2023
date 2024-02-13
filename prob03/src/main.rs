use std::fs::read_to_string;

fn main() {
    let input = read_to_string("inputs/input03.txt").unwrap();
    let engine = Engine::from_input(&input);
    engine.part1();
    engine.part2();
}

struct Engine {
    schematic: Vec<Vec<char>>,
}

#[derive(Debug, Eq, PartialEq)]
enum PartType {
    Number,
    Symbol,
    Period,
}

impl PartType {
    fn parse_char(c: char) -> Self {
        match c {
            '0'..='9' => Self::Number,
            '.' => Self::Period,
            _ => Self::Symbol,
        }
    }
}

impl Engine {
    fn from_input(input: &str) -> Self {
        let mut schematic = vec![];
        for line in input.lines() {
            let row = line.chars().collect();
            schematic.push(row);
        }
        Self { schematic }
    }

    fn part1(&self) {
        let rows = self.schematic.len();
        let cols = self.schematic[0].len();
        let mut ans = 0;
        for i in 0..rows {
            let mut j = 0;
            while j < cols {
                match PartType::parse_char(self.schematic[i][j]) {
                    PartType::Number => {
                        let mut k = j + 1;
                        while k < cols
                            && PartType::parse_char(self.schematic[i][k]) == PartType::Number
                        {
                            k += 1;
                        }
                        if self.is_part_number(i, j, k - 1) {
                            let num: String = (&self.schematic[i][j..k]).iter().collect();
                            let num: usize = num.parse().unwrap();
                            ans += num;
                        }
                        j = k;
                    }
                    _ => j += 1,
                }
            }
        }
        println!("{}", ans);
    }

    fn is_part_number(&self, r: usize, c1: usize, c2: usize) -> bool {
        let rows = self.schematic.len();
        let cols = self.schematic[0].len();
        if r > 0 {
            if c1 > 0 && PartType::parse_char(self.schematic[r - 1][c1 - 1]) == PartType::Symbol {
                return true;
            }
            for i in c1..=c2 {
                if PartType::parse_char(self.schematic[r - 1][i]) == PartType::Symbol {
                    return true;
                }
            }
            if c2 < cols - 1
                && PartType::parse_char(self.schematic[r - 1][c2 + 1]) == PartType::Symbol
            {
                return true;
            }
        }
        if c1 > 0 && PartType::parse_char(self.schematic[r][c1 - 1]) == PartType::Symbol {
            return true;
        }
        if c2 < cols - 1 && PartType::parse_char(self.schematic[r][c2 + 1]) == PartType::Symbol {
            return true;
        }
        if r < rows - 1 {
            if c1 > 0 && PartType::parse_char(self.schematic[r + 1][c1 - 1]) == PartType::Symbol {
                return true;
            }
            for i in c1..=c2 {
                if PartType::parse_char(self.schematic[r + 1][i]) == PartType::Symbol {
                    return true;
                }
            }
            if c2 < cols - 1
                && PartType::parse_char(self.schematic[r + 1][c2 + 1]) == PartType::Symbol
            {
                return true;
            }
        }
        false
    }

    fn part2(&self) {
        let mut ans = 0;
        let rows = self.schematic.len();
        let cols = self.schematic[0].len();
        for i in 0..rows {
            for j in 0..cols {
                if self.schematic[i][j] == '*' {
                    let adj_numbers = self.find_adj_numbers(i, j);
                    if adj_numbers.len() == 2 {
                        ans += adj_numbers[0] * adj_numbers[1];
                    }
                }
            }
        }
        println!("{}", ans);
    }

    fn find_adj_numbers(&self, r: usize, c: usize) -> Vec<usize> {
        let mut ans = vec![];
        let rows = self.schematic.len();
        let cols = self.schematic[0].len();
        if r > 0 {
            if PartType::parse_char(self.schematic[r - 1][c]) == PartType::Number {
                ans.push(self.get_number(r - 1, c));
            } else {
                if c > 0 && PartType::parse_char(self.schematic[r - 1][c - 1]) == PartType::Number {
                    ans.push(self.get_number(r - 1, c - 1));
                }
                if c < cols - 1
                    && PartType::parse_char(self.schematic[r - 1][c + 1]) == PartType::Number
                {
                    ans.push(self.get_number(r - 1, c + 1));
                }
            }
        }
        if c > 0 && PartType::parse_char(self.schematic[r][c - 1]) == PartType::Number {
            ans.push(self.get_number(r, c - 1));
        }
        if c < cols - 1 && PartType::parse_char(self.schematic[r][c + 1]) == PartType::Number {
            ans.push(self.get_number(r, c + 1));
        }

        if r < rows - 1 {
            if PartType::parse_char(self.schematic[r + 1][c]) == PartType::Number {
                ans.push(self.get_number(r + 1, c));
            } else {
                if c > 0 && PartType::parse_char(self.schematic[r + 1][c - 1]) == PartType::Number {
                    ans.push(self.get_number(r + 1, c - 1));
                }
                if c < cols - 1
                    && PartType::parse_char(self.schematic[r + 1][c + 1]) == PartType::Number
                {
                    ans.push(self.get_number(r + 1, c + 1));
                }
            }
        }
        ans
    }

    fn get_number(&self, r: usize, c: usize) -> usize {
        let mut lo: i32 = c as i32;
        let mut hi = c + 1;
        while lo >= 0 && PartType::parse_char(self.schematic[r][lo as usize]) == PartType::Number {
            lo -= 1;
        }
        let lo = (lo + 1) as usize;
        while hi < self.schematic[0].len()
            && PartType::parse_char(self.schematic[r][hi]) == PartType::Number
        {
            hi += 1;
        }

        let num: String = (&self.schematic[r][lo..hi]).iter().collect();
        num.parse().unwrap()
    }
}
