use std::fs::read_to_string;

fn main() {
    let input = read_to_string("inputs/input13.txt").unwrap();
    part1(&input, 0);
    part1(&input, 1);
}

fn transpose<T>(v: &Vec<Vec<T>>) -> Vec<Vec<T>>
where
    T: Clone,
{
    assert!(!v.is_empty());
    (0..v[0].len())
        .map(|i| v.iter().map(|inner| inner[i].clone()).collect::<Vec<T>>())
        .collect()
}

fn part1(input: &str, allowed_diff: usize) {
    let mut cur = vec![];
    let mut ans = 0;
    for line in input.lines() {
        if line.is_empty() {
            ans += calc_score(&cur, allowed_diff);
            cur = vec![];
        } else {
            let row: Vec<char> = line.chars().collect();
            cur.push(row);
        }
    }
    ans += calc_score(&cur, allowed_diff);
    println!("part1 = {}", ans);
}

fn calc_score(matrix: &Vec<Vec<char>>, allowed_diff: usize) -> usize {
    let mut ans = 0;
    let row_mirror = find_mirror_in_row(matrix, allowed_diff);
    ans += row_mirror * 100;
    let trans = transpose(matrix);
    ans += find_mirror_in_row(&trans, allowed_diff);
    ans
}

fn find_mirror_in_row(matrix: &Vec<Vec<char>>, allowed_diff: usize) -> usize {
    for i in 1..matrix.len() as i32 {
        let mut up = i - 1;
        let mut down = i;
        let mut diff = 0;
        while up >= 0 && (down as usize) < matrix.len() {
            diff += diff_count(&matrix[up as usize], &matrix[down as usize]);
            if diff > allowed_diff {
                break;
            }
            up -= 1;
            down += 1;
        }
        if (up < 0 || (down as usize) >= matrix.len()) && diff == allowed_diff {
            return i as usize;
        }
    }
    0
}

fn diff_count(r1: &[char], r2: &[char]) -> usize {
    let mut ans = 0;
    for i in 0..r1.len() {
        if r1[i] != r2[i] {
            ans += 1;
        }
    }
    ans
}
