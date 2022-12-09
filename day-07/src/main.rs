use std::collections::HashMap;
use std::fs;
use std::time::Instant;

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

// - keep track of current directory in Stack
// - keep track of directory sizes in HashMap
// - on cd => set current dir
// - on ls =>
//  - if hashmap entry for current dir exists => do nothing
//  - loop through entries in dir and sum file sizes
//  - loop through all parent dirs in stack and increment the size of their entries in the hashmap

fn part1(input: String) -> HashMap<String, i32> {
    let mut current_dir: Vec<&str> = Vec::new();
    let mut dirs: HashMap<String, i32> = HashMap::new();

    let mut lines = input.lines().peekable();

    while let Some(line) = lines.next() {
        let split = line.split(" ").collect::<Vec<&str>>();

        match split[0] {
            "$" => match split[1] {
                "cd" => match split[2] {
                    ".." => {
                        current_dir.pop();
                    }
                    dir_name => {
                        current_dir.push(dir_name);
                    }
                },
                "ls" => {
                    let dir = dir_string(&current_dir);

                    if dirs.contains_key(&dir) {
                        ()
                    }

                    let mut children = Vec::new();

                    while let Some(l) = lines.next_if(|l| !l.starts_with("$")) {
                        children.push(l);
                    }

                    let sum: i32 = children
                        .iter()
                        .filter_map(|c| {
                            let mut split = c.split(" ");
                            let first = split.next().unwrap();

                            return first.parse::<i32>().ok();
                        })
                        .sum();

                    for i in 1..=current_dir.len() {
                        let dir = dir_string(&current_dir[..i]);

                        if dir == "" {
                            ()
                        }

                        dirs.entry(dir).and_modify(|v| *v += sum).or_insert(sum);
                    }
                }
                _ => (),
            },
            _ => (),
        }
    }

    let total = dirs.values().fold(0, |total, v| {
        if v <= &100_000 {
            return total + v;
        }

        total
    });

    println!("{total}");

    dirs
}

fn part2(input: String) {
    let dirs = part1(input.clone());

    let total_space = 70_000_000;
    let needed_space = 30_000_000;
    let root_size = dirs.get("/").unwrap();
    let space_available = total_space - root_size;
    let actual_needed = needed_space - space_available;

    let mut current_best_size = root_size;

    for size in dirs.values() {
        if size >= &actual_needed && size < current_best_size {
            current_best_size = size
        }
    }

    println!("{current_best_size}")
}

fn dir_string(dir: &[&str]) -> String {
    if dir.len() == 1 {
        return dir[0].to_string();
    }

    dir[1..].join("/")
}
