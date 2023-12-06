use regex::Regex;

fn main() {
    let input = include_str!("day01.txt");

    // println!("{input}");

    // Part 1
    // let mut sum = 0;
    //
    // for line in input.lines() {
    //     let mut numbers = line.chars().filter(|it| it.is_numeric());
    //     let first_num = numbers.next().unwrap();
    //     let mut numbers = line.chars().filter(|it| it.is_numeric());
    //     let last_num = numbers.last().unwrap();
    //
    //     sum += format!("{first_num}{last_num}").parse::<u32>().unwrap();
    // }
    //
    // println!("numeric {sum}");

    // Part 2
    // I hate my life
    let mut sum = 0;
    let re = Regex::new(r"([0-9]|one|two|three|four|five|six|seven|eight|nine)").unwrap();

    for line in input.lines() {
        let mut offset = 0;

        let mut first = None;
        let mut last = None;

        loop {
            let Some(digit) = re.find_at(line, offset) else {
                break;
            };

            offset = digit.start() + 1;

            let digit = digit.as_str();

            let number = match digit.parse::<u32>() {
                Ok(num) => num,
                Err(_) => match digit {
                    "one" => 1,
                    "two" => 2,
                    "three" => 3,
                    "four" => 4,
                    "five" => 5,
                    "six" => 6,
                    "seven" => 7,
                    "eight" => 8,
                    "nine" => 9,
                    _ => panic!(),
                },
            };

            if first.is_none() {
                first = Some(number);
            }

            last = Some(number);
        }

        sum += first.unwrap() * 10 + last.unwrap();
    }

    println!("numeric & word {sum}");
}
