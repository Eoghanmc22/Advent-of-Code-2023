use std::time::Instant;

fn main() {
    let start = Instant::now();

    let input = include_str!("day02.txt");

    let mut sum = 0;
    'outer: for line in input.lines() {
        let (header, results) = line.split_once(':').unwrap();

        let game = header[5..].parse::<u32>().unwrap();

        let mut red_needed = 0;
        let mut green_needed = 0;
        let mut blue_needed = 0;

        for round in results.split(';') {
            let mut red = 0;
            let mut green = 0;
            let mut blue = 0;

            for cube in round.split(',') {
                let (num, color) = cube.trim().split_once(' ').unwrap();
                match color.trim() {
                    "red" => {
                        red += num.parse::<u32>().unwrap();
                    }
                    "green" => {
                        green += num.parse::<u32>().unwrap();
                    }
                    "blue" => {
                        blue += num.parse::<u32>().unwrap();
                    }
                    _ => panic!(),
                }
            }

            // if red > 12 {
            //     continue 'outer;
            // }
            // if green > 13 {
            //     continue 'outer;
            // }
            // if blue > 14 {
            //     continue 'outer;
            // }

            red_needed = red.max(red_needed);
            green_needed = green.max(green_needed);
            blue_needed = blue.max(blue_needed);
        }
        sum += red_needed * blue_needed * green_needed;
        // sum += game
    }

    let elapsed = start.elapsed();
    println!("{sum}, {}us", elapsed.as_micros());
}
