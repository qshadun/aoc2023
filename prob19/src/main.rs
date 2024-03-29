use std::{
    collections::{HashMap, VecDeque},
    fs::read_to_string,
};

fn main() {
    let input = read_to_string("inputs/input19.txt").unwrap();
    part1(&input);
    part2(&input);
}

fn part1(input: &str) {
    let game = Game::from_input(input);
    println!("{}", game.part1());
}

fn part2(input: &str) {
    let game = Game::from_input(input);
    println!("{}", game.part2());
}

#[derive(Debug, Clone, Copy)]
enum Destination<'a> {
    Label(&'a str),
    Accepted,
    Rejected,
}

impl<'a> Destination<'a> {
    fn from_str(s: &'a str) -> Self {
        match s {
            "A" => Destination::Accepted,
            "R" => Destination::Rejected,
            label => Destination::Label(label),
        }
    }
}

#[derive(Debug)]
enum Rule<'a> {
    ConditionJump {
        condition: (char, char, i32),
        dest: Destination<'a>,
    },
    Jump {
        dest: Destination<'a>,
    },
}

impl<'a> Rule<'a> {
    fn from_str(s: &'a str) -> Self {
        let ps: Vec<_> = s.split(':').collect();
        if ps.len() == 1 {
            Rule::Jump {
                dest: Destination::from_str(ps[0]),
            }
        } else {
            let q = ps[0].chars().nth(0).unwrap();
            let comp = ps[0].chars().nth(1).unwrap();
            let v: i32 = ps[0][2..].parse().unwrap();

            Rule::ConditionJump {
                condition: (q, comp, v),
                dest: Destination::from_str(ps[1]),
            }
        }
    }

    fn is_match(&self, part: &Part) -> Option<Destination> {
        match self {
            Rule::ConditionJump { dest, condition } => match condition {
                ('x', '<', v) => {
                    if part.x < *v {
                        Some(*dest)
                    } else {
                        None
                    }
                }
                ('x', '>', v) => {
                    if part.x > *v {
                        Some(*dest)
                    } else {
                        None
                    }
                }
                ('m', '<', v) => {
                    if part.m < *v {
                        Some(*dest)
                    } else {
                        None
                    }
                }
                ('m', '>', v) => {
                    if part.m > *v {
                        Some(*dest)
                    } else {
                        None
                    }
                }
                ('a', '<', v) => {
                    if part.a < *v {
                        Some(*dest)
                    } else {
                        None
                    }
                }
                ('a', '>', v) => {
                    if part.a > *v {
                        Some(*dest)
                    } else {
                        None
                    }
                }
                ('s', '<', v) => {
                    if part.s < *v {
                        Some(*dest)
                    } else {
                        None
                    }
                }
                ('s', '>', v) => {
                    if part.s > *v {
                        Some(*dest)
                    } else {
                        None
                    }
                }
                _ => {
                    panic!("invalid rule")
                }
            },
            Rule::Jump { dest } => Some(*dest),
        }
    }
}

#[derive(Debug)]
struct WorkFlow<'a> {
    name: &'a str,
    rules: Vec<Rule<'a>>,
}

impl<'a> WorkFlow<'a> {
    fn from_line(line: &'a str) -> Self {
        let open_brac = line.find('{').unwrap();
        let name = &line[..open_brac];
        let rules_str = &line[(open_brac + 1)..(line.len() - 1)];
        let rule_str_vec: Vec<_> = rules_str.split(',').collect();
        let rules: Vec<_> = rule_str_vec.into_iter().map(Rule::from_str).collect();
        Self { name, rules }
    }

    fn match_part(&self, part: &Part) -> Destination {
        for rule in self.rules.iter() {
            if let Some(dest) = rule.is_match(part) {
                return dest;
            }
        }
        panic!("no match rule");
    }
}

#[derive(Debug, Clone, Copy)]
struct Part {
    x: i32,
    m: i32,
    a: i32,
    s: i32,
}

impl Part {
    fn from_line(line: &str) -> Self {
        let ps: Vec<_> = line[1..(line.len() - 1)].split(',').collect();
        let x: i32 = ps[0][2..].parse().unwrap();
        let m: i32 = ps[1][2..].parse().unwrap();
        let a: i32 = ps[2][2..].parse().unwrap();
        let s: i32 = ps[3][2..].parse().unwrap();
        Self { x, m, a, s }
    }

    fn score(&self) -> i32 {
        self.x + self.m + self.a + self.s
    }
}

fn reverse_condition(condition: (char, char, i32)) -> (char, char, i32) {
    let (var, comp, v) = condition;
    match comp {
        '>' => (var, '<', v + 1),
        '<' => (var, '>', v - 1),
        _ => panic!("invalid comp"),
    }
}

fn count_one_var(conditions: Vec<(char, i32)>) -> i64 {
    let mut min = 0;
    let mut max = 4001;
    for (comp, v) in conditions {
        match comp {
            '<' => max = max.min(v),
            '>' => min = min.max(v),
            _ => panic!("invalid comp"),
        }
    }
    (max - min - 1).max(0) as i64
}

fn count(conditions: &Vec<(char, char, i32)>) -> i64 {
    let mut xs = vec![];
    let mut ms = vec![];
    let mut aas = vec![];
    let mut ss = vec![];
    for &(var, comp, v) in conditions {
        match var {
            'x' => xs.push((comp, v)),
            'm' => ms.push((comp, v)),
            'a' => aas.push((comp, v)),
            's' => ss.push((comp, v)),
            _ => {}
        }
    }
    count_one_var(xs) * count_one_var(ms) * count_one_var(aas) * count_one_var(ss)
}

#[derive(Debug)]
struct Game<'a> {
    workflows: HashMap<&'a str, WorkFlow<'a>>,
    parts: Vec<Part>,
}

impl<'a> Game<'a> {
    fn from_input(input: &'a str) -> Self {
        let mut workflows = HashMap::new();
        let mut parts = vec![];
        let mut is_parts = false;
        for line in input.lines() {
            if line.is_empty() {
                is_parts = true;
            } else if is_parts {
                parts.push(Part::from_line(line));
            } else {
                let wf = WorkFlow::from_line(line);
                workflows.insert(wf.name, wf);
            }
        }
        Self { workflows, parts }
    }

    fn handle_part(&self, part: &Part) -> bool {
        let mut cur_dest = Destination::Label("in");
        loop {
            match cur_dest {
                Destination::Accepted => return true,
                Destination::Rejected => return false,
                Destination::Label(label) => {
                    let wf = self.workflows.get(label).unwrap();
                    cur_dest = wf.match_part(part);
                }
            }
        }
    }

    fn part1(&self) -> i32 {
        let mut score = 0;
        for p in self.parts.iter() {
            if self.handle_part(p) {
                score += p.score();
            }
        }
        score
    }

    #[allow(clippy::type_complexity)]
    fn part2(&self) -> i64 {
        let mut ans = 0;
        let mut q: VecDeque<(Destination, Vec<(char, char, i32)>)> = VecDeque::new();
        q.push_back((Destination::Label("in"), vec![]));
        while let Some((cur, mut path)) = q.pop_front() {
            if count(&path) == 0 {
                continue;
            }
            match cur {
                Destination::Label(cur_label) => {
                    let wf = self.workflows.get(cur_label).unwrap();
                    for rule in wf.rules.iter() {
                        match rule {
                            Rule::ConditionJump { condition, dest } => {
                                let mut take = path.clone();
                                take.push(*condition);
                                q.push_back((*dest, take));
                                path.push(reverse_condition(*condition));
                            }
                            Rule::Jump { dest } => {
                                q.push_back((*dest, path));
                                break;
                            }
                        }
                    }
                }
                Destination::Accepted => {
                    ans += count(&path);
                }
                _ => {}
            }
        }
        ans
    }
}
