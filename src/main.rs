use aoc_lib::solutions::{Day01, Day02, Day03, Day04, Day05, Day06, Day07, Day08, Day09, Solution};
use std::fs;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let day: u8 = args.get(1).and_then(|s| s.parse().ok()).unwrap_or(1);

    let input =
        fs::read_to_string(format!("inputs/day{:02}.txt", day)).expect("Input file not found");
    let input = input.trim_end_matches("\n");

    let solution: Box<dyn Solution> = match day {
        1 => Box::new(Day01),
        2 => Box::new(Day02),
        3 => Box::new(Day03),
        4 => Box::new(Day04),
        5 => Box::new(Day05),
        6 => Box::new(Day06),
        7 => Box::new(Day07),
        8 => Box::new(Day08),
        9 => Box::new(Day09),
        _ => panic!("Day {} not implemented", day),
    };

    println!("Part 1 Solution: {}", solution.part1(&input));
    println!("Part 2 Solution: {}", solution.part2(&input));
}
