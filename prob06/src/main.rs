fn main() {
    let input = "Time:        63     78     94     68
Distance:   411   1274   2047   1035";
    part1(input);

    part2(input);
}

fn part1(input: &str) {
    let lines: Vec<_> = input.lines().collect();
    let times = parse_numbers(lines[0]);
    let distances = parse_numbers(lines[1]);
    println!("{:?} \n{:?}", times, distances);
    let mut ans = 1i64;
    for i in 0..times.len() {
        let mut win_ways = 0i64;
        let t = times[i];
        let d = distances[i];
        for tt in 0..=t {
            if tt * (t - tt) > d {
                win_ways += 1;
            }
        }
        ans *= win_ways;
    }
    println!("part1={ans}");
}
fn parse_numbers(line: &str) -> Vec<i64> {
    let mut ans = vec![];
    let parts = line.split(' ');
    for p in parts {
        if let Ok(num) = p.parse::<i64>() {
            ans.push(num);
        }
    }
    ans
}

fn parse_number2(line: &str) -> u64 {
    let colon = line.find(':').unwrap();
    let s = &line[colon + 1..];
    let s = s.replace(' ', "");
    s.parse().unwrap()
}
fn part2(input: &str) {
    let lines: Vec<_> = input.lines().collect();
    let time = parse_number2(lines[0]);
    let dist = parse_number2(lines[1]);
    println!("{} {}", time, dist);
    let mut l = 1u64;
    let mut r = time / 2;
    while l <= r {
        let mid = (l + r) / 2;
        let d = mid * (time - mid);
        if d >= dist {
            r = mid - 1;
        } else {
            l = mid + 1;
        }
    }
    let ans = time - 2 * l + 1;
    println!("part2={}", ans);
}
