use std::{collections::VecDeque, time::Instant};

use ahash::HashSet;

fn main() {
    let start = Instant::now();

    let input = include_str!("day04.txt");

    // let mut sum = 0;
    // for line in input.lines() {
    //     let (_card, data) = line.split_once(": ").unwrap();
    //     let (winning, current) = data.split_once(" | ").unwrap();
    //     let winning = winning
    //         .split(" ")
    //         .flat_map(|it| it.trim().parse::<u32>())
    //         .collect::<HashSet<_>>();
    //     let current = current
    //         .split(" ")
    //         .flat_map(|it| it.trim().parse::<u32>())
    //         .collect::<HashSet<_>>();
    //
    //     let count = winning.intersection(&current).count();
    //
    //     if count != 0 {
    //         sum += 2u32.pow(count as u32 - 1);
    //     }
    // }

    let mut cards = Vec::default();
    for (idx, line) in input.lines().enumerate() {
        let (_card, data) = line.split_once(": ").unwrap();
        let (winning, won) = data.split_once(" | ").unwrap();
        let winning = winning
            .split(" ")
            .flat_map(|it| it.trim().parse::<u32>())
            .collect::<HashSet<_>>();
        let won = won
            .split(" ")
            .flat_map(|it| it.trim().parse::<u32>())
            .collect::<HashSet<_>>();

        let count = winning.intersection(&won).count();

        cards.push(Card {
            // winning,
            // won,
            idx,
            count,
        })
    }

    let mut queue = cards.iter().collect::<VecDeque<_>>();
    let mut count = 0;
    while let Some(next) = queue.pop_front() {
        for card in next.idx + 1..=next.idx + next.count {
            queue.push_back(&cards[card]);
        }

        count += 1;
    }

    let elapsed = start.elapsed();
    println!("{count}, {}us", elapsed.as_micros());
}

struct Card {
    // winning: HashSet<u32>,
    // won: HashSet<u32>,
    idx: usize,
    count: usize,
}
