use crate::solutions::Solution;
use crate::utils::parser::{self, Parser, StrParser};
use itertools::Itertools;

#[derive(Debug, Clone, Copy)]
pub struct RedTile {
    row: u64,
    col: u64,
}

impl RedTile {
    fn area_against(&self, other: &RedTile) -> u64 {
        (self.row.abs_diff(other.row) + 1) * (self.col.abs_diff(other.col) + 1)
    }
}

pub struct Sol;

impl Solution for Sol {
    type Parsed = Vec<RedTile>;

    fn parser(&self) -> impl Parser<&str, Output = Self::Parsed> {
        parser::from_str::<u64>
            .split_array(",")
            .map(|[row, col]| RedTile { row, col })
            .lines()
    }

    fn part1(&self, tiles: &Self::Parsed) -> String {
        tiles
            .iter()
            .tuple_combinations()
            .map(|(t1, t2)| t1.area_against(&t2))
            .max()
            .unwrap()
            .to_string()
    }

    fn part2(&self, tiles: &Self::Parsed) -> String {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::solutions::{check_part1, check_part2};

    const TEST_INPUT: &str = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";

    #[test]
    fn test_part1() {
        check_part1(&Sol, TEST_INPUT, "50");
    }

    #[test]
    fn test_part2() {
        check_part2(&Sol, TEST_INPUT, "25272");
    }
}
