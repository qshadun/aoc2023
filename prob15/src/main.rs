use std::fs::read_to_string;

fn main() {
    let input = read_to_string("inputs/input15.txt").unwrap();
    part1(&input);
    part2(&input);
}

fn hash(s: &str) -> usize {
    let mut ans = 0;
    for c in s.chars() {
        ans += c as usize;
        ans *= 17;
        ans %= 256;
    }
    ans
}

fn part1(input: &str) {
    let mut ans = 0;
    for line in input.lines() {
        let words: Vec<&str> = line.split(',').collect();
        for w in words {
            ans += hash(w);
        }
    }
    println!("part1 = {}", ans);
}

fn part2(input: &str) {
    let mut boxes: Vec<Vec<Lens>> = vec![vec![]; 256];
    for line in input.lines() {
        let words: Vec<&str> = line.split(',').collect();
        for w in words {
            match Operation::from_word(w) {
                Operation::Remove(lens) => {
                    let pos = hash(&lens.label);
                    if let Some(idx) = boxes[pos].iter().position(|x| x.label == lens.label) {
                        boxes[pos].remove(idx);
                    }
                }
                Operation::Add(lens) => {
                    let pos = hash(&lens.label);
                    if let Some(idx) = boxes[pos].iter().position(|x| x.label == lens.label) {
                        boxes[pos][idx] = lens;
                    } else {
                        boxes[pos].push(lens);
                    }
                }
            }
        }
    }

    println!("part2 = {}", calc_power(boxes));
}

fn calc_power(boxes: Vec<Vec<Lens>>) -> usize {
    let mut ans = 0;
    for (i, b) in boxes.iter().enumerate() {
        for (j, lens) in b.iter().enumerate() {
            ans += (i + 1) * (j + 1) * lens.focus;
        }
    }
    ans
}

#[derive(Debug)]
enum Operation {
    Remove(Lens),
    Add(Lens),
}

impl Operation {
    fn from_word(w: &str) -> Self {
        if w.ends_with('-') {
            let label = w[0..w.len() - 1].to_string();
            Self::Remove(Lens { label, focus: 0 })
        } else {
            Self::Add(Lens::from_word(w))
        }
    }
}
#[derive(Debug, Clone)]
struct Lens {
    label: String,
    focus: usize,
}

impl Lens {
    fn from_word(w: &str) -> Self {
        let parts: Vec<_> = w.split('=').collect();
        Self {
            label: parts[0].to_string(),
            focus: parts[1].parse().unwrap(),
        }
    }
}

impl PartialEq for Lens {
    fn eq(&self, other: &Self) -> bool {
        self.label == other.label
    }
}

impl Eq for Lens {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash() {
        assert_eq!(hash("HASH"), 52);
    }

    #[test]
    fn print_hash() {
        println!("rn = {}", hash("rn"));
        println!("cm = {}", hash("cm"));
        println!("qp = {}", hash("qp"));
    }
}
