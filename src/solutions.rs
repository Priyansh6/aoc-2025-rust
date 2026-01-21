mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;

pub use {
    day01::Day01, day02::Day02, day03::Day03, day04::Day04, day05::Day05, day06::Day06,
    day07::Day07, day08::Day08, day09::Day09,
};

pub trait Solution {
    fn part1(&self, input: &str) -> String;
    fn part2(&self, input: &str) -> String;
}
