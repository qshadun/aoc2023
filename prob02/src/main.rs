use std::collections::HashMap;
use std::fs::read_to_string;

fn main() {
    let limit: HashMap<&str, usize> = HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);

    let input = read_to_string("inputs/input02.txt").unwrap();

    part1(&input, &limit);
    part2(&input);
}

fn part1(input: &str, limit: &HashMap<&str, usize>) {
    let mut ans = 0;
    for line in input.lines() {
        let parts: Vec<_> = line.split(':').collect();
        let id_parts: Vec<_> = parts[0].split(' ').collect();
        let id: usize = id_parts[1].parse().unwrap();
        let mut valid = true;
        'outer: for game in parts[1].split("; ") {
            let ball_parts: Vec<_> = game.split(", ").collect();
            for ball in ball_parts {
                let one_ball_parts: Vec<_> = ball.trim().split(' ').collect();
                let color = one_ball_parts[1];
                let cnt: usize = one_ball_parts[0].trim().parse().unwrap();
                if cnt > *limit.get(color).unwrap() {
                    valid = false;
                    break 'outer;
                }
            }
        }
        if valid {
            ans += id;
        }
    }
    println!("{}", ans);
}

fn part2(input: &str) {
    let mut ans = 0;
    for line in input.lines() {
        ans += power(line)
    }
    println!("{}", ans);
}

fn power(line: &str) -> u64 {
    let parts: Vec<_> = line.split(':').collect();
    let mut max_cnt: HashMap<&str, u64> = HashMap::from([("red", 0), ("blue", 0), ("green", 0)]);
    for game in parts[1].split("; ") {
        let ball_parts: Vec<_> = game.split(", ").collect();
        for ball in ball_parts {
            let one_ball_parts: Vec<_> = ball.trim().split(' ').collect();
            let color = one_ball_parts[1];
            let cnt: u64 = one_ball_parts[0].trim().parse().unwrap();
            if cnt > *(max_cnt.get(color).unwrap()) {
                max_cnt.insert(color, cnt);
            }
        }
    }
    let mut ans = 1;
    for &x in max_cnt.values() {
        ans *= x;
    }
    ans
}
