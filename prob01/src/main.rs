use lazy_static::lazy_static;
use regex;
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
        static ref re: regex::Regex = regex::Regex::new(r"\d").unwrap();
    }
    let l = re.find(line).unwrap();

    let mut ans: usize = l.as_str().parse().unwrap();
    let rev_line: String = line.chars().rev().collect();
    let r = re.find(&rev_line).unwrap();
    ans * 10 + r.as_str().parse::<usize>().unwrap()
}

const numbers: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];
static mut rev_numbers: Vec<String> = vec![];

fn reverse_numbers() {
    unsafe {
        for &num in numbers.iter() {
            let r: String = num.chars().rev().collect();
            rev_numbers.push(r);
        }
    }
}
fn get_num2(line: &str) -> usize {
    let (mut idx, mut d1) = get_digit(line);
    for (i, num_str) in numbers.iter().enumerate() {
        match line.find(num_str) {
            Some(pos) => {
                if pos < idx {
                    idx = pos;
                    d1 = i + 1;
                }
            }
            None => {}
        }
    }

    let reversed: String = line.chars().rev().collect();
    let (mut idx2, mut d2) = get_digit(&reversed);
    unsafe {
        for (i, num_str) in rev_numbers.iter().enumerate() {
            match reversed.find(num_str) {
                Some(pos) => {
                    if pos < idx2 {
                        idx2 = pos;
                        d2 = i + 1;
                    }
                }
                None => {}
            }
        }
    }

    d1 * 10 + d2
}

fn get_digit(line: &str) -> (usize, usize) {
    lazy_static! {
        static ref re: regex::Regex = regex::Regex::new(r"\d").unwrap();
    }
    let l = re.find(line).unwrap();
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
