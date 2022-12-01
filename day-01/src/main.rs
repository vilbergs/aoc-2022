use std::fs;

mod utils;

fn main() {
    let filename = 
        //"./sample"
        "./input";

    let input = fs::read_to_string(filename).expect("Could not read input");

    part1(input.clone());
    part2(input);
}

fn part1(input: String) {
    unimplemented!()
}

fn part2(input: String) {
    unimplemented!()
}
