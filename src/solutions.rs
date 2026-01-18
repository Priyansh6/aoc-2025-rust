pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;
pub mod day07;
pub mod day08;

pub use {
    day01::Day01, day02::Day02, day03::Day03, day04::Day04, day05::Day05, day06::Day06,
    day07::Day07, day08::Day08,
};

pub trait Solution {
    fn part1(&self, input: &str) -> String;
    fn part2(&self, input: &str) -> String;
}
