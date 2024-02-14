use std::collections::VecDeque;
use std::fs::read_to_string;

fn main() {
    let input = read_to_string("inputs/input05.txt").unwrap();
    let garden = Garden::from_input(&input);
    garden.part1();
    garden.part2();
}




#[derive(Debug)]
struct Garden {
    seeds: Vec<i64>,
    mappings: Vec<Vec<Mapping>>
}

impl Garden {
    fn from_input(input: &str) -> Self {
        let mut seeds = vec![];
        let mut mappings = vec![];
        let lines: Vec<_> = input.lines().collect();
        for p in lines[0]["seeds: ".len()..].split(' ') {
            seeds.push(p.parse().unwrap());
        }
        let mut i = 1;
        while i < lines.len() {
            if lines[i].is_empty() {
                i += 1;
                continue;
            }
            if lines[i].contains("map") {
                let mut new_map = vec![];
                let mut j = i + 1;
                while j < lines.len() && !lines[j].is_empty() {
                    new_map.push(Mapping::from_line(lines[j]));
                    j += 1;
                }
                new_map.sort_by_key(|x| x.source_start);
                mappings.push(new_map);
                i = j;
            }
        }
        Self {
            seeds, mappings
        }
    }

    fn part1(&self) {
        let mut cur = self.seeds.clone();
        for cur_map in self.mappings.iter() {
            for i in 0..cur.len() {
                let source = cur[i];
                for m in cur_map {
                    if source < m.source_start {
                        break;
                    } else if source >= m.source_start && source < m.source_start + m.range {
                        cur[i] = m.target_start + source - m.source_start;
                    }
                }
            }
        }
        let ans = cur.iter().min().unwrap();
        println!("{}", ans);
    }

    fn part2(&self) {
        let mut cur_range: Vec<Range> = vec![];
        for i in 0..self.seeds.len()/2 {
            cur_range.push(Range::new(self.seeds[2*i as usize], self.seeds[2*i as usize + 1]));
        }
        for cur_map in self.mappings.iter() {
            cur_range = Self::transform(cur_range, cur_map);
        }
        let ans = cur_range.iter().min_by_key(|x| x.start).unwrap().start;
        println!("{}", ans);
    }

    fn transform(mut cur_range: Vec<Range>, cur_map: &[Mapping]) -> Vec<Range> {
        let mut new_range = vec![];
        cur_range.sort_by_key(|r| r.start);
        let mut q: VecDeque<Range> = VecDeque::new();
        for r in cur_range {
            q.push_back(r);
        }
        let mut i = 0;
        while let Some(r) = q.pop_front() {
            while i < cur_map.len() && cur_map[i].source_start + cur_map[i].range - 1 < r.start {
                i += 1;
            }
            if i == cur_map.len() {
                new_range.push(r);
            } else {
               if r.start + r.range - 1 < cur_map[i].source_start {
                   new_range.push(r);
               } else {
                   let intersect_start = r.start.max(cur_map[i].source_start);
                   if intersect_start > r.start {
                       new_range.push(Range::new(r.start, intersect_start - r.start));
                   }
                   let intersect_end = (r.start + r.range - 1).min(cur_map[i].source_start + cur_map[i].range - 1);
                   if intersect_end < r.start + r.range - 1 {
                       q.push_front(Range::new(intersect_end + 1, r.start + r.range - intersect_end));
                   }
                   let mapped_start = cur_map[i].target_start + r.start - cur_map[i].source_start;
                   new_range.push(Range::new(mapped_start, intersect_end - intersect_start + 1));
               }
            }

        }
        new_range
    }
}

#[derive(Debug, Clone)]
struct Range {
    start: i64,
    range: i64,
}

impl Range {
    fn new(start: i64, range: i64) -> Self {
        Self {start, range}
    }
}
#[derive(Debug, Clone)]
struct Mapping {
    source_start: i64,
    target_start: i64,
    range: i64,
}

impl Mapping {
    fn from_line(line: &str) -> Self {
        let parts: Vec<_> = line.split(' ').collect();
        Self {
            source_start: parts[1].trim().parse().unwrap(),
            target_start: parts[0].trim().parse().unwrap(),
            range: parts[2].trim().parse().unwrap(),
        }
    }
}