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

// Left can be contained in right
// Right can be contained in left
// partial overlap doesn't matter

// 1. get lines
// 2. split each line into pairs
// 3. split each pair into their range
// 4. if left contains right -> increase a counter
// 5. else if right contains left -> increase a counter

fn part1(input: String) {
    let pairs = input
        .lines()
        .map(|line| {
            let mut parts = line.split([',', '-']);
            let left = (
                str_to_i32(parts.next().unwrap()),
                str_to_i32(parts.next().unwrap()),
            );
            let right = (
                str_to_i32(parts.next().unwrap()),
                str_to_i32(parts.next().unwrap()),
            );

            (left, right)
        })
        .collect::<Vec<((i32, i32), (i32, i32))>>();

    let mut counter = 0;

    for ((left_1, left_2), (right_1, right_2)) in pairs {
        if left_1 <= right_1 && left_2 >= right_2 || right_1 <= left_1 && right_2 >= left_2 {
            counter += 1
        }
    }

    println!("{counter}")
}

fn part2(input: String) {
    let pairs = input
        .lines()
        .map(|line| {
            let mut parts = line.split([',', '-']);
            let left = (
                str_to_i32(parts.next().unwrap()),
                str_to_i32(parts.next().unwrap()),
            );
            let right = (
                str_to_i32(parts.next().unwrap()),
                str_to_i32(parts.next().unwrap()),
            );

            (left, right)
        })
        .collect::<Vec<((i32, i32), (i32, i32))>>();

    let mut counter = 0;

    for ((left_1, left_2), (right_1, right_2)) in pairs {
        let left_range = left_1..=left_2;
        let right_range = right_1..=right_2;

        let right_overlaps_with_left = left_range.clone().any(|l_n| right_range.contains(&l_n));
        let left_overlaps_with_right = right_range.clone().any(|r_n| left_range.contains(&r_n));

        if left_overlaps_with_right || right_overlaps_with_left {
            counter += 1
        }
    }

    println!("{counter}")
}

fn str_to_i32(str: &str) -> i32 {
    str.parse::<i32>().expect("Could not parse str to i32")
}
