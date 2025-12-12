mod day1;
mod day10;
mod day11;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod graph;
mod grid;
mod utils;
use core::panic;
use std::{env, time::Instant};

const PART1_SOLS: &[fn(&str)] = &[
    day1::part1,
    day2::part1,
    day3::part1,
    day4::part1,
    day5::part1,
    day6::part1,
    day7::part1,
    day8::part1,
    day9::part1,
    day10::part1,
    day11::part1,
];
const PART2_SOLS: &[fn(&str)] = &[
    day1::part2,
    day2::part2,
    day3::part2,
    day4::part2,
    day5::part2,
    day6::part2,
    day7::part2,
    day8::part2,
    day9::part2,
    day10::part2,
    day11::part2,
];

fn main() {
    let args: Vec<String> = env::args().collect();
    let day = if let Ok(d) = &args[1].parse::<usize>() {
        *d
    } else {
        panic!("Could not parse day argument. Arguments : {:?}", args);
    };

    let now = Instant::now();
    let input_filename: &str = &format!("inputs/{day}");
    let elapsed = now.elapsed();
    println!("Input file read in {elapsed:.2?}");
    println!();

    let now = Instant::now();
    println!("Part 1");
    PART1_SOLS[day - 1](&utils::read_to_string(input_filename));
    let elapsed = now.elapsed();
    println!("Executed in {elapsed:.2?}");
    println!();

    let now = Instant::now();
    println!("Part 2");
    PART2_SOLS[day - 1](&utils::read_to_string(input_filename));
    let elapsed = now.elapsed();
    println!("Executed in {elapsed:.2?}");
}
