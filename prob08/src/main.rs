use std::collections::HashMap;
use std::fs::read_to_string;

fn main() {
    let input = read_to_string("inputs/input08.txt").unwrap();
    let lines: Vec<_> = input.lines().collect();
    let moves: Vec<char> = lines[0].chars().collect();
    let mut map: HashMap<&str, (&str, &str)> = HashMap::new();
    for line in &lines[2..] {
        let (from, (left, right)) = parse_line(line);
        map.insert(from, (left, right));
    }

    let start = "AAA";
    println!("part1 = {:?}", part1(&moves, &map, start, |x| x == "ZZZ"));
    part2(&moves, &map);
}

fn part1(
    moves: &[char],
    map: &HashMap<&str, (&str, &str)>,
    start: &str,
    end: fn(&str) -> bool,
) -> usize {
    let mut steps = 0;

    let mut cur = start;
    loop {
        for c in moves.iter() {
            match c {
                'L' => {
                    cur = map.get(cur).unwrap().0;
                }
                'R' => {
                    cur = map.get(cur).unwrap().1;
                }
                _ => panic!("invalid move {c}"),
            }
            steps += 1;
            if end(cur) {
                break;
            }
        }
        if end(cur) {
            break;
        }
    }
    steps
}

fn parse_line(line: &str) -> (&str, (&str, &str)) {
    let from = &line[..3];
    let left = &line[7..10];
    let right = &line[12..15];
    (from, (left, right))
}

fn part2(moves: &[char], map: &HashMap<&str, (&str, &str)>) {
    let start_nodes: Vec<&str> = map
        .keys()
        .filter(|x| x.chars().nth(2).unwrap() == 'A')
        .copied()
        .collect();
    let mut steps_vec = vec![];
    for node in start_nodes {
        let steps = part1(moves, map, node, |x| x.chars().nth(2).unwrap() == 'Z');
        steps_vec.push(steps);
    }
    println!("{:?}", steps_vec);
    println!("{:?}", lcm(&steps_vec));
}

pub fn lcm(nums: &[usize]) -> usize {
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = lcm(&nums[1..]);
    a * b / gcd_of_two_numbers(a, b)
}

fn gcd_of_two_numbers(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }
    gcd_of_two_numbers(b, a % b)
}
