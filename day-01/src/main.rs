use std::fs;

fn main() {
    let filename = // "./input/sample";
    "./input/input";

    let input = fs::read_to_string(filename).expect("Could not read input");

    part1(input.clone());
    part2(input);
}

// Find the nth elf (i + 1) carrying the most weight
// 1. Loop through input
// 2. Sum calories until a newline appears and push to vector
// 3. Find the max of the vector

fn part1(input: String) {
    let lines = input.lines();

    let mut accumulator = 0;
    let mut current_max = 0;

    for line in lines {
        if line == "" {
            if current_max < accumulator {
                current_max = accumulator
            }

            accumulator = 0;
            continue;
        }

        let num = line
            .parse::<i32>()
            .expect("Could not parse line into number");

        accumulator += num;
    }

    println!("{current_max}");
}

fn part2(input: String) {
    let lines = input.lines().enumerate().collect::<Vec<(usize, &str)>>();

    let mut accumulator = 0;
    let mut top_3: [i32; 3] = [0, 0, 0];

    for (i, line) in lines.clone() {
        if line == "" || i == lines.len() - 1 {
            if i == lines.len() - 1 {
                let num = line
                    .parse::<i32>()
                    .expect("Could not parse line into number");
                accumulator += num;
            }

            let min_val = top_3.iter().min().unwrap().clone();
            let min_index = top_3
                .iter()
                .position(|val| val == &min_val)
                .unwrap()
                .clone();

            if min_val < accumulator {
                top_3[min_index as usize] = accumulator
            }

            accumulator = 0;
            continue;
        }

        let num = line
            .parse::<i32>()
            .expect("Could not parse line into number");
        accumulator += num;
    }

    println!("{:?}", top_3.into_iter().sum::<i32>());
}
