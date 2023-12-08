use std::{mem, time::Instant};

use itertools::Itertools;

fn main() {
    let start = Instant::now();

    let input = include_str!("day05.txt");

    let mut sections = input.split("\n\n");

    let mut vals = sections
        .next()
        .unwrap()
        .split_once(": ")
        .unwrap()
        .1
        .split(" ")
        .map(|it| it.trim().parse::<u32>())
        .flatten()
        .tuples()
        .map(|(a, b)| a..a + b)
        .collect_vec();

    for map in sections {
        // Parse map
        let mut maps = vec![];
        for entry in map.lines().skip(1) {
            let mut iter = entry.split(" ").map(|it| it.trim().parse::<u32>().unwrap());

            let dest_start = iter.next().unwrap();
            let src_start = iter.next().unwrap();
            let len = iter.next().unwrap();

            maps.push((src_start, dest_start, len));
        }

        // for val in &mut vals {
        //     for map in &maps {
        //         if (map.0..map.0 + map.2).contains(val) {
        //             *val = *val - map.0 + map.1;
        //         }
        //     }
        // }

        let mut old_vals = mem::take(&mut vals);
        'outer: while let Some(mut val) = old_vals.pop() {
            for map in &maps {
                let map_range = map.0..map.0 + map.2;

                if map_range.contains(&val.start) && map_range.contains(&(val.end - 1)) {
                    //good
                    val.start -= map.0;
                    val.start += map.1;
                    val.end -= map.0;
                    val.end += map.1;
                } else if map_range.contains(&val.start) {
                    old_vals.push(map_range.end..val.end);

                    val.start -= map.0;
                    val.start += map.1;
                    val.end = map.1 + map.2;
                } else if map_range.contains(&(val.end - 1)) {
                    old_vals.push(val.start..map_range.start);

                    val.start = map.1;
                    val.end -= map.0;
                    val.end += map.1;
                } else if val.contains(&map_range.start) && val.contains(&map_range.end) {
                    old_vals.push(val.start..map_range.start);
                    old_vals.push(map_range.end..val.end);

                    val = map.1..map.1 + map.2;
                } else {
                    continue;
                }

                vals.push(val);
                continue 'outer;
            }

            vals.push(val);
        }
    }

    let rst = vals.iter().map(|it| it.start).min().unwrap();

    let elapsed = start.elapsed();
    println!("{rst}, {}us", elapsed.as_micros());
}
