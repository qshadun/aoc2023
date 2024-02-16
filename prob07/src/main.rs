use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter, Write};
use std::fs::read_to_string;

fn main() {
    let input = read_to_string("inputs/input07.txt").unwrap();
    let game = Game::from_input(&input);
    game.part1();
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl HandType {
    fn from_hand(cards: &[Card; 5]) -> Self {
        let mut counter: HashMap<Card, u8> = HashMap::new();
        // part1
        // for c in cards.iter() {
        //     *counter.entry(*c).or_default() += 1;
        // }
        // let mut values: Vec<u8> = counter.values().copied().collect();
        // part2 start
        let mut wild_card_cnt = 0;
        for c in cards.iter() {
            match c {
                Card::Jack => wild_card_cnt += 1,
                _ => {
                    *counter.entry(*c).or_default() += 1;
                }
            }
        }
        let mut values: Vec<u8> = counter.values().copied().collect();
        values.sort();
        if values.is_empty() {
            return Self::FiveOfAKind;
        }
        let last_idx = values.len() - 1;
        values[last_idx] += wild_card_cnt;
        // part2 end

        if values.len() == 1 {
            Self::FiveOfAKind
        } else if values.len() == 2 && values[1] == 4 {
            Self::FourOfAKind
        } else if values.len() == 2 && values[1] == 3 {
            Self::FullHouse
        } else if values.len() == 3 && values[2] == 3 {
            Self::ThreeOfAKind
        } else if values.len() == 3 && values[1] == 2 {
            Self::TwoPair
        } else if values.len() == 4 {
            Self::OnePair
        } else {
            Self::HighCard
        }
    }
}
#[derive(Ord, PartialOrd, Eq, PartialEq, Copy, Clone, Hash)]
enum Card {
    Jack, // comment for part1
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    // Jack, // uncomment for part1
    Queen,
    King,
    Ace,
}

impl Card {
    fn from_char(c: char) -> Self {
        match c {
            '2' => Self::Two,
            '3' => Self::Three,
            '4' => Self::Four,
            '5' => Self::Five,
            '6' => Self::Six,
            '7' => Self::Seven,
            '8' => Self::Eight,
            '9' => Self::Nine,
            'T' => Self::Ten,
            'J' => Self::Jack,
            'Q' => Self::Queen,
            'K' => Self::King,
            'A' => Self::Ace,
            _ => panic!("illegal card {}", c),
        }
    }

    fn display_char(&self) -> char {
        match self {
            Self::Two => '2',
            Card::Three => '3',
            Card::Four => '4',
            Card::Five => '5',
            Card::Six => '6',
            Card::Seven => '7',
            Card::Eight => '8',
            Card::Nine => '9',
            Card::Ten => 'T',
            Card::Jack => 'J',
            Card::Queen => 'Q',
            Card::King => 'K',
            Card::Ace => 'A',
        }
    }
}

impl Debug for Card {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_char(self.display_char())
    }
}

impl Display for Card {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_char(self.display_char())
    }
}

#[derive(Ord, PartialOrd, Eq, PartialEq)]
struct Hand {
    hand_type: HandType,
    cards: [Card; 5],
}
impl Hand {
    fn from_cards(cards: &str) -> Self {
        let chars: Vec<_> = cards.chars().collect();
        let cards: Vec<Card> = chars[0..5].iter().map(|&x| Card::from_char(x)).collect();
        let cards: [Card; 5] = cards.try_into().unwrap();
        let hand_type = HandType::from_hand(&cards);
        Self { hand_type, cards }
    }
}

impl Debug for Hand {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}{}{}{}",
            self.cards[0], self.cards[1], self.cards[2], self.cards[3], self.cards[4]
        )
    }
}

impl Display for Hand {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug)]
struct Game {
    hand_and_bids: Vec<(Hand, usize)>,
}

impl Game {
    fn from_input(input: &str) -> Self {
        let mut hand_and_bids = vec![];
        for line in input.lines() {
            let parts: Vec<_> = line.split(' ').collect();
            let hand = Hand::from_cards(parts[0]);
            let bid: usize = parts[1].trim().parse().unwrap();
            hand_and_bids.push((hand, bid));
        }
        hand_and_bids.sort();
        Self { hand_and_bids }
    }

    fn part1(&self) {
        let mut ans = 0;
        for i in 0..self.hand_and_bids.len() {
            ans += (i + 1) * self.hand_and_bids[i].1
        }
        println!("{:?}", self);
        println!("part1={ans}");
    }
}
