use lazy_static::lazy_static;
use std::fs::read_to_string;

fn main() {
    let input = read_to_string("inputs/input01.txt").unwrap();
    part1(&input);
    reverse_numbers();
    part2(&input);
}

fn part1(input: &str) {
    let mut ans = 0;
    for line in input.lines() {
        ans += get_num1(line);
    }
    println!("{}", ans);
}

fn get_num1(line: &str) -> usize {
    lazy_static! {
        static ref RE: regex::Regex = regex::Regex::new(r"\d").unwrap();
    }
    let l = RE.find(line).unwrap();

    let ans: usize = l.as_str().parse().unwrap();
    let rev_line: String = line.chars().rev().collect();
    let r = RE.find(&rev_line).unwrap();
    ans * 10 + r.as_str().parse::<usize>().unwrap()
}

const NUMBERS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];
static mut REV_NUMBERS: Vec<String> = vec![];

fn reverse_numbers() {
    unsafe {
        for &num in NUMBERS.iter() {
            let r: String = num.chars().rev().collect();
            REV_NUMBERS.push(r);
        }
    }
}
fn get_num2(line: &str) -> usize {
    let (mut idx, mut d1) = get_digit(line);
    for (i, num_str) in NUMBERS.iter().enumerate() {
        if let Some(pos) = line.find(num_str) {
            if pos < idx {
                idx = pos;
                d1 = i + 1;
            }
        }
    }

    let reversed: String = line.chars().rev().collect();
    let (mut idx2, mut d2) = get_digit(&reversed);
    unsafe {
        for (i, num_str) in REV_NUMBERS.iter().enumerate() {
            if let Some(pos) = reversed.find(num_str) {
                if pos < idx2 {
                    idx2 = pos;
                    d2 = i + 1;
                }
            }
        }
    }

    d1 * 10 + d2
}

fn get_digit(line: &str) -> (usize, usize) {
    lazy_static! {
        static ref RE: regex::Regex = regex::Regex::new(r"\d").unwrap();
    }
    let l = RE.find(line).unwrap();
    let d: usize = l.as_str().parse().unwrap();
    (l.start(), d)
}

fn part2(input: &str) {
    let mut ans = 0;
    for line in input.lines() {
        ans += get_num2(line);
    }
    println!("{}", ans);
}
