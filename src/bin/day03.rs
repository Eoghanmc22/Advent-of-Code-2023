#![feature(test)]
extern crate test;

use std::{
    cell::RefCell,
    mem,
    sync::atomic::{AtomicBool, Ordering},
    time::Instant,
};

use itertools::Itertools;

fn main() {
    let start = Instant::now();

    let (part_1, part_2) = implementation();

    println!("part 1: {part_1}");
    println!("part 2: {part_2}");

    println!("{}us", start.elapsed().as_micros());
}

fn implementation() -> (u32, u32) {
    let input = include_str!("day03.txt");

    let mut numbers = Vec::default();
    let mut symbols = Vec::default();

    let mut row = 0;
    let mut column = 0;
    let mut number = 0;
    let mut number_len = 0;
    let mut numbers_subbuf = Vec::default();
    let mut symbols_subbuf = Vec::default();
    for (byte, next_byte) in input.bytes().tuple_windows() {
        match byte as char {
            digit_char @ '0'..='9' => {
                number *= 10;
                number += digit_char.to_digit(10).unwrap();
                number_len += 1;

                if !(next_byte as char).is_numeric() {
                    numbers_subbuf.push(Number {
                        num: number,
                        already_counted: false.into(),
                        row,
                        column: column + 1 - number_len,
                        length: number_len,
                    });

                    number = 0;
                    number_len = 0;
                }

                column += 1;
            }
            '.' => {
                // No action
                column += 1;
            }
            '\n' => {
                // Move cursor to next line
                column = 0;
                row += 1;

                numbers.push(mem::take(&mut numbers_subbuf));
                symbols.push(mem::take(&mut symbols_subbuf));
            }
            sym => {
                symbols_subbuf.push(Symbol { sym, row, column });

                column += 1;
            }
        }
    }
    numbers.push(mem::take(&mut numbers_subbuf));
    symbols.push(mem::take(&mut symbols_subbuf));

    let mut sum_1 = 0;

    for symbol in symbols.iter().flatten() {
        [
            symbol.row.checked_sub(1),
            Some(symbol.row),
            symbol.row.checked_add(1),
        ]
        .into_iter()
        .flatten()
        .flat_map(|it| numbers.get(it))
        .flat_map(|it| it.iter())
        .filter(|it| {
            it.column <= symbol.column + 1 && symbol.column + 1 <= it.column + it.length + 1
        })
        .for_each(|it| {
            if !it.already_counted.load(Ordering::Relaxed) {
                sum_1 += it.num;
            }

            it.already_counted.store(true, Ordering::Relaxed)
        });
    }

    let mut sum_2 = 0;

    for symbol in symbols.iter().flatten() {
        if symbol.sym == '*' {
            let tuple = [
                symbol.row.checked_sub(1),
                Some(symbol.row),
                symbol.row.checked_add(1),
            ]
            .into_iter()
            .flatten()
            .flat_map(|it| numbers.get(it))
            .flat_map(|it| it.iter())
            .filter(|it| {
                it.column <= symbol.column + 1 && symbol.column + 1 <= it.column + it.length + 1
            })
            .collect_tuple();

            if let Some((a, b)) = tuple {
                sum_2 += a.num * b.num;
            }
        }
    }

    (sum_1, sum_2)
}

#[derive(Debug)]
struct Number {
    num: u32,
    already_counted: AtomicBool,

    row: usize,
    column: usize,
    length: usize,
}
#[derive(Debug)]
struct Symbol {
    sym: char,

    row: usize,
    column: usize,
}

#[bench]
fn name(b: &mut test::Bencher) {
    b.iter(|| implementation())
}
