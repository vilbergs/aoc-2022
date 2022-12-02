use std::fs;
use std::time::Instant;

fn main() {
    let filename = // "./sample.txt";
    "./input.txt";

    let input = fs::read_to_string(filename).expect("Could not read input");

    part1(input.clone());
    part2(input);
}

enum Shape {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

enum Result {
    Loss = 0,
    Draw = 3,
    Win = 6,
}

fn part1(input: String) {
    println!("=======================");
    println!("Running Part 1");
    println!("=======================");

    let now = Instant::now();

    let results: i32 = input.lines().fold(0, |total, l| {
        let mut raw_choices = l.split(" ");

        let opponent_choice = match raw_choices.next().unwrap() {
            "A" => Some(Shape::Rock),
            "B" => Some(Shape::Paper),
            "C" => Some(Shape::Scissors),
            _ => None,
        }
        .unwrap();

        let my_choice = match raw_choices.next().unwrap() {
            "X" => Some(Shape::Rock),
            "Y" => Some(Shape::Paper),
            "Z" => Some(Shape::Scissors),
            _ => None,
        }
        .unwrap();

        let result = match (opponent_choice, my_choice) {
            (Shape::Rock, Shape::Paper) => Shape::Paper as i32 + Result::Win as i32,
            (Shape::Paper, Shape::Scissors) => Shape::Scissors as i32 + Result::Win as i32,
            (Shape::Scissors, Shape::Rock) => Shape::Rock as i32 + Result::Win as i32,
            (Shape::Paper, Shape::Rock) => Shape::Rock as i32 + Result::Loss as i32,
            (Shape::Scissors, Shape::Paper) => Shape::Paper as i32 + Result::Loss as i32,
            (Shape::Rock, Shape::Scissors) => Shape::Scissors as i32 + Result::Loss as i32,
            (_, mine) => mine as i32 + Result::Draw as i32,
        };

        total + result
    });

    println!("{results}");

    let elapsed = now.elapsed();
    println!("-----------------------");
    println!("Part 1 ran in: {:.2?}", elapsed);
    println!("-----------------------");
}

fn part2(input: String) {
    println!("=======================");
    println!("Running Part 2");
    println!("=======================");

    let now = Instant::now();

    let results = input.lines().fold(0, |total, l| {
        let mut raw_choices = l.split(" ");

        let opponent_choice = match raw_choices.next().unwrap() {
            "A" => Some(Shape::Rock),
            "B" => Some(Shape::Paper),
            "C" => Some(Shape::Scissors),
            _ => None,
        }
        .unwrap();

        let desired_result = match raw_choices.next().unwrap() {
            "X" => Some(Result::Loss),
            "Y" => Some(Result::Draw),
            "Z" => Some(Result::Win),
            _ => None,
        }
        .unwrap();

        let result = match (opponent_choice, desired_result) {
            (Shape::Rock, Result::Win) => Shape::Paper as i32 + Result::Win as i32,
            (Shape::Paper, Result::Win) => Shape::Scissors as i32 + Result::Win as i32,
            (Shape::Scissors, Result::Win) => Shape::Rock as i32 + Result::Win as i32,
            (Shape::Rock, Result::Loss) => Shape::Scissors as i32 + Result::Loss as i32,
            (Shape::Paper, Result::Loss) => Shape::Rock as i32 + Result::Loss as i32,
            (Shape::Scissors, Result::Loss) => Shape::Paper as i32 + Result::Loss as i32,
            (opponent, draw) => opponent as i32 + draw as i32,
        };

        total + result
    });

    println!("{results}");

    let elapsed = now.elapsed();
    println!("-----------------------");
    println!("Part 2 ran in: {:.2?}", elapsed);
    println!("-----------------------");
}
