use std::fs;
use std::time::Instant;

fn main() {
    let filename = // "./sample.txt";
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

fn part1(input: String) {
    let chars = input.char_indices();

    let mut result = 0;

    for (i, _) in chars {
        let start_index = match i.checked_sub(4) {
            Some(n) => n,
            _ => 0,
        };

        let slice = &input[start_index..i];

        if slice.len() < 4 {
            continue;
        }

        let has_duplicates = slice.chars().any(|c| slice.matches(c).count() >= 2);

        if !has_duplicates {
            result = i;

            break;
        }
    }

    println!("{result}");
}

fn part2(input: String) {
    let chars = input.char_indices();

    let mut result = 0;

    for (i, _) in chars {
        let start_index = match i.checked_sub(14) {
            Some(n) => n,
            _ => 0,
        };

        let slice = &input[start_index..i];

        if slice.len() < 14 {
            continue;
        }

        let has_duplicates = slice.chars().any(|c| slice.matches(c).count() >= 2);

        if !has_duplicates {
            result = i;

            break;
        }
    }

    println!("{result}");
}
