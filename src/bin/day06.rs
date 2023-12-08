use std::time::Instant;

fn main() {
    let start = Instant::now();

    let input = include_str!("day06.txt");

    let mut lines = input.lines();
    // let times = lines
    //     .next()
    //     .unwrap()
    //     .split_whitespace()
    //     .filter(|it| !it.is_empty())
    //     .map(|it| it.trim().parse::<f32>())
    //     .flatten()
    //     .collect_vec();
    // let dists = lines
    //     .next()
    //     .unwrap()
    //     .split_whitespace()
    //     .filter(|it| !it.is_empty())
    //     .map(|it| it.trim().parse::<f32>())
    //     .flatten()
    //     .collect_vec();
    let time = lines
        .next()
        .unwrap()
        .split_once(": ")
        .unwrap()
        .1
        .split_whitespace()
        .filter(|it| !it.is_empty())
        .collect::<String>()
        .parse::<f32>()
        .unwrap();
    let dist = lines
        .next()
        .unwrap()
        .split_once(": ")
        .unwrap()
        .1
        .split_whitespace()
        .filter(|it| !it.is_empty())
        .collect::<String>()
        .parse::<f32>()
        .unwrap();

    let mut rst = 1;

    // for (time, dist) in times.iter().zip(dists) {
    let sqrt = (time * time - 4.0 * dist).sqrt();
    let a = (-time + sqrt) / -2.0;
    let b = (-time - sqrt) / -2.0;

    let min = a.min(b).floor() as u32;
    let max = a.max(b).ceil() as u32;

    rst *= (max - min) - 1;
    // }

    println!("{rst}, {}", start.elapsed().as_micros());
}
