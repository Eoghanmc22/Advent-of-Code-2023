use std::{cmp::Ordering, convert::Infallible, str::FromStr};

use ahash::HashMap;
use itertools::Itertools;

fn main() {
    let input = include_str!("day07.txt");

    let mut cards = vec![];
    for line in input.lines() {
        let (hand, bid) = line.split_once(" ").unwrap();
        let bid = bid.parse::<u32>().unwrap();

        cards.push((hand, bid));
    }

    cards.sort_by(|(a, _), (b, _)| compare(a, b));

    println!("{cards:#?}");

    let sum: u32 = cards
        .iter()
        .enumerate()
        .map(|(idx, (_, bid))| (idx + 1) as u32 * bid)
        .sum();

    println!("sum {sum}");
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug, Hash)]
#[repr(u8)]
enum TypeOfHand {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl FromStr for TypeOfHand {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut buckets = HashMap::default();
        for char in s.chars() {
            *buckets.entry(char).or_insert(0usize) += 1;
        }

        let scorer = |buckets: &HashMap<char, usize>| match buckets.len() {
            1 => TypeOfHand::FiveOfAKind,
            2 => {
                let max = buckets.values().max().unwrap();
                if *max == 4 {
                    TypeOfHand::FourOfAKind
                } else {
                    TypeOfHand::FullHouse
                }
            }
            3 => {
                let max = buckets.values().max().unwrap();
                if *max == 3 {
                    TypeOfHand::ThreeOfAKind
                } else {
                    TypeOfHand::TwoPair
                }
            }
            4 => TypeOfHand::OnePair,
            _ => TypeOfHand::HighCard,
        };

        let mut score = (scorer)(&buckets);

        if let Some(jokers) = buckets.remove(&'J') {
            let cards = ['2', '3', '4', '5', '6', '7', '8', '9', 'T', 'Q', 'K', 'A'];

            cards
                .into_iter()
                .combinations_with_replacement(jokers)
                .for_each(|replacements| {
                    let mut buckets = buckets.clone();

                    for replacement in replacements {
                        *buckets.entry(replacement).or_insert(0usize) += 1;
                    }

                    let new_score = (scorer)(&buckets);
                    if new_score > score {
                        score = new_score
                    }
                })
        }

        Ok(score)
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug, Hash)]
enum Card {
    J,
    N2,
    N3,
    N4,
    N5,
    N6,
    N7,
    N8,
    N9,
    T,
    // J,
    Q,
    K,
    A,
}

impl From<char> for Card {
    fn from(c: char) -> Self {
        match c {
            '2' => Card::N2,
            '3' => Card::N3,
            '4' => Card::N4,
            '5' => Card::N5,
            '6' => Card::N6,
            '7' => Card::N7,
            '8' => Card::N8,
            '9' => Card::N9,
            'T' => Card::T,
            'J' => Card::J,
            'Q' => Card::Q,
            'K' => Card::K,
            'A' => Card::A,
            _ => panic!("Bad card: {c}"),
        }
    }
}

fn compare(a: &str, b: &str) -> Ordering {
    let type_a = a.parse::<TypeOfHand>();
    let type_b = b.parse::<TypeOfHand>();

    type_a.cmp(&type_b).then_with(|| {
        for (char_a, char_b) in a.chars().zip(b.chars()) {
            let card_a: Card = char_a.into();
            let card_b: Card = char_b.into();

            let ord = card_a.cmp(&card_b);

            if ord != Ordering::Equal {
                return ord;
            }
        }
        Ordering::Equal
    })
}
