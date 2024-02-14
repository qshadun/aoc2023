use std::fs::read_to_string;

fn main() {
    let input = read_to_string("inputs/input04.txt").unwrap();
    part1(&input);
    part2(&input);
}

fn part1(input: &str) {
    let mut ans = 0;
    for line in input.lines() {
        let card = Card::from_line(line);
        ans += card.score();
    }
    println!("{}", ans);
}

fn part2(input: &str) {
    let game = Game::from_input(input);
    game.play();
}

struct Game {
    cards: Vec<Card>,
}

impl Game {
    fn from_input(input: &str) -> Self {
        let mut cards = vec![];
        for line in input.lines() {
            cards.push(Card::from_line(line));
        }
        Self { cards }
    }

    fn play(&self) {
        let n = self.cards.len();
        let mut card_cnt = vec![1usize; n];
        let mut ans = 0;
        for i in 0..n {
            let cnt = card_cnt[i];
            ans += cnt;
            let win_cnt = self.cards[i].win_count();
            for j in 1..win_cnt + 1 {
                card_cnt[i + j] += cnt;
            }
        }
        println!("{}", ans);
    }
}
#[derive(Debug, Clone)]
struct Card {
    winning_numbers: Vec<i32>,
    numbers: Vec<i32>,
}
impl Card {
    fn from_line(line: &str) -> Self {
        // Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
        let parts: Vec<_> = line.split(" | ").collect();
        let win_parts: Vec<_> = parts[0].trim().split(": ").collect();
        let winning_numbers = Self::parse_numbers(win_parts[1]);
        let numbers = Self::parse_numbers(parts[1]);
        Self {
            winning_numbers,
            numbers,
        }
    }

    fn parse_numbers(s: &str) -> Vec<i32> {
        let parts: Vec<_> = s.trim().split(' ').collect();
        let mut ans = vec![];
        for p in parts {
            if p.is_empty() {
                continue;
            }
            let n: i32 = p.trim().parse().unwrap();
            ans.push(n);
        }
        ans
    }

    fn score(&self) -> i32 {
        let cnt = self.win_count();
        if cnt > 0 {
            2i32.pow(cnt as u32 - 1)
        } else {
            0
        }
    }

    fn win_count(&self) -> usize {
        self.numbers
            .iter()
            .filter(|x| self.winning_numbers.contains(x))
            .count()
    }
}
