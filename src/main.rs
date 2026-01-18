use aoc_lib::solutions;
use std::fs;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let day: u8 = args.get(1).and_then(|s| s.parse().ok()).unwrap_or(1);

    let input =
        fs::read_to_string(format!("inputs/day{:02}.txt", day)).expect("Input file not found");
    let input = input.trim_end_matches("\n");

    let solution: Box<dyn solutions::Solution> = match day {
        1 => Box::new(solutions::day01::Day01),
        2 => Box::new(solutions::day02::Day02),
        3 => Box::new(solutions::day03::Day03),
        4 => Box::new(solutions::day04::Day04),
        5 => Box::new(solutions::day05::Day05),
        6 => Box::new(solutions::day06::Day06),
        7 => Box::new(solutions::day07::Day07),
        8 => Box::new(solutions::day08::Day08),
        _ => panic!("Day {} not implemented", day),
    };

    println!("Part 1 Solution: {}", solution.part1(&input));
    println!("Part 2 Solution: {}", solution.part2(&input));
}
