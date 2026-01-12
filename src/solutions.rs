pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
mod day05;

pub trait Solution {
    fn part1(&self, input: &str) -> String;
    fn part2(&self, input: &str) -> String;
}
