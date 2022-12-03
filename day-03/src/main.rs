use std::fs;
use std::time::Instant;

static LOWERCASE_ASCII_OFFSET: u32 = 96;
static UPPERCASE_ASCII_OFFSET: u32 = 38;

fn main() {
    let filename = //"./sample.txt";
    "./input.txt";

    let input = fs::read_to_string(filename).expect("Could not read input");
    let input_part_2 = input.clone(); // Cloning input here so it doesn't affect benchmarks

    println!("=======================");
    println!("Running Part 1");
    println!("=======================");

    let now = Instant::now();

    part1(input);

    let elapsed = now.elapsed();
    println!("-----------------------");
    println!("Part 1 ran in: {:.2?}", elapsed);
    println!("-----------------------");

    println!("=======================");
    println!("Running Part 2");
    println!("=======================");

    let now = Instant::now();

    part2(input_part_2);

    let elapsed = now.elapsed();
    println!("-----------------------");
    println!("Part 2 ran in: {:.2?}", elapsed);
    println!("-----------------------");
}

// 1. Fold through input
// 2. Split each line in two
// 3. Loop through first split and find the item that exists in second split
// 4. find ascii decimal code and subtract it
// 6. If it is uppercase, multiply the priority by two
// 6. add it to the total

fn part1(input: String) {
    let lines = input.lines().fold(0, |total, line| {
        let halfway = line.len() / 2;
        let first_half = &line[..halfway];
        let second_half = &line[halfway..];

        let duplicate = first_half
            .chars()
            .find(|c| second_half.contains(&c.to_string()));

        let priority = if let Some(d) = duplicate {
            if d.is_ascii_uppercase() {
                d as u32 - UPPERCASE_ASCII_OFFSET
            } else {
                d as u32 - LOWERCASE_ASCII_OFFSET
            }
        } else {
            total
        };

        total + priority
    });

    println!("{lines}")
}

fn part2(input: String) {
    let lines = input
        .lines()
        .collect::<Vec<&str>>()
        .chunks(3)
        .fold(0, |total, chunk| {
            let duplicate = chunk[0]
                .chars()
                .find(|c| chunk[1].contains(&c.to_string()) && chunk[2].contains(&c.to_string()));

            let priority = if let Some(d) = duplicate {
                if d.is_ascii_uppercase() {
                    d as u32 - UPPERCASE_ASCII_OFFSET
                } else {
                    d as u32 - LOWERCASE_ASCII_OFFSET
                }
            } else {
                total
            };

            total + priority
        });

    println!("{lines}")
}
