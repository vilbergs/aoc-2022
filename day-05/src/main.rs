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

// - Build a vector of stacks from starting positions
// - parse rearrangement procedure into an iterator of tuples of 3 numbers
// - loop through iterator, pop x from one stack and push x to the other.
//
fn part1(input: String) {
    let mut lines = input.lines();
    let mut stacks = lines
        .by_ref()
        .take_while(|l| l != &"")
        .collect::<Vec<&str>>();

    let last_row = stacks.pop().unwrap();

    let mut parsed_stacks = Vec::new();

    for (i, index_col) in last_row.chars().enumerate() {
        let mut stack = Vec::new();

        if index_col.is_numeric() {
            for row in stacks.clone() {
                let cols = row.chars().collect::<Vec<char>>();

                if let Some(char) = cols.get(i) {
                    if char.is_alphabetic() {
                        stack.insert(0, char.clone());
                    }
                }
            }

            parsed_stacks.push(stack);
        }
    }

    let moves = lines.map(|l| {
        let split = l
            .split(" ")
            .filter_map(|word| word.parse::<i32>().ok())
            .collect::<Vec<i32>>();

        (split[0], split[1], split[2])
    });

    println!("{:?}", parsed_stacks);

    for (count, from, to) in moves {
        println!("{} {} {}", count, from, to);

        let mut from_stack = parsed_stacks.get((from - 1) as usize).unwrap().clone();
        let mut to_stack = parsed_stacks.get((to - 1) as usize).unwrap().clone();

        for _ in 0..count {
            if let Some(item) = from_stack.pop() {
                to_stack.push(item);
            }
        }

        parsed_stacks[(from - 1) as usize] = from_stack;
        parsed_stacks[(to - 1) as usize] = to_stack;
    }

    let tops = parsed_stacks.iter().fold(String::new(), |mut str, stack| {
        if let Some(char) = stack.last() {
            str.push(char.clone());
        }

        str
    });

    println!("{tops}");
}

fn part2(input: String) {
    let mut lines = input.lines();
    let mut stacks = lines
        .by_ref()
        .take_while(|l| l != &"")
        .collect::<Vec<&str>>();

    let last_row = stacks.pop().unwrap();

    let mut parsed_stacks = Vec::new();

    for (i, index_col) in last_row.chars().enumerate() {
        let mut stack = Vec::new();

        if index_col.is_numeric() {
            for row in stacks.clone() {
                let cols = row.chars().collect::<Vec<char>>();

                if let Some(char) = cols.get(i) {
                    if char.is_alphabetic() {
                        stack.insert(0, char.clone())
                    }
                }
            }

            parsed_stacks.push(stack);
        }
    }

    let moves = lines.map(|l| {
        let split = l
            .split(" ")
            .filter_map(|word| word.parse::<i32>().ok())
            .collect::<Vec<i32>>();

        (split[0], split[1], split[2])
    });

    println!("{:?}", parsed_stacks);

    for (count, from, to) in moves {
        println!("{} {} {}", count, from, to);

        let mut from_stack = parsed_stacks.get((from - 1) as usize).unwrap().clone();
        let mut to_stack = parsed_stacks.get((to - 1) as usize).unwrap().clone();

        let slice_start = match from_stack.len().checked_sub(count as usize) {
            Some(n) => n,
            _ => 0,
        };

        let items = &from_stack[slice_start..];

        to_stack.extend_from_slice(items);
        let trunk = match from_stack.len().checked_sub(count as usize) {
            Some(n) => n,
            _ => 0,
        };
        from_stack.truncate(trunk);

        parsed_stacks[(from - 1) as usize] = from_stack;
        parsed_stacks[(to - 1) as usize] = to_stack;
    }

    let tops = parsed_stacks.iter().fold(String::new(), |mut str, stack| {
        if let Some(char) = stack.last() {
            str.push(char.clone());
        }

        str
    });

    println!("{tops}");
}
