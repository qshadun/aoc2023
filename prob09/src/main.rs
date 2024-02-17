use std::fs::read_to_string;

fn main() {
    let input = read_to_string("inputs/input09.txt").unwrap();
    part1_and_2(&input);
}

fn part1_and_2(input: &str) {
    let mut ans1 = 0;
    let mut ans2 = 0;
    for line in input.lines() {
        let seq = parse_line(line);
        let (a1, a2) = predict(seq);
        ans1 += a1;
        ans2 += a2;
    }
    println!("part1 = {}", ans1);
    println!("part2 = {}", ans2);
}

fn parse_line(line: &str) -> Vec<i64> {
    let mut ans = vec![];
    let parts: Vec<&str> = line.split(' ').collect();
    for p in parts {
        if !p.is_empty() {
            let num: i64 = p.trim().parse().unwrap();
            ans.push(num)
        }
    }
    ans
}

fn predict(seq: Vec<i64>) -> (i64, i64) {
    let mut seqs = vec![];
    seqs.push(seq);
    loop {
        let mut next_seq = vec![];
        let last_seq = &seqs[seqs.len() - 1];
        for i in 0..last_seq.len() - 1 {
            next_seq.push(last_seq[i + 1] - last_seq[i]);
        }
        if next_seq.iter().all(|x| *x == 0) {
            break;
        }
        seqs.push(next_seq);
    }
    let mut ans1 = 0;
    let mut ans2 = 0;
    for sq in seqs.iter().rev() {
        ans1 += sq[sq.len() - 1];
        ans2 = sq[0] - ans2;
    }
    (ans1, ans2)
}
