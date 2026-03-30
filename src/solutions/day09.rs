use crate::solutions::Solution;
use crate::utils::parser::{self, Parser};
use itertools::Itertools;

#[derive(Debug, Clone, Copy)]
struct RedTile {
    row: u64,
    col: u64,
}

impl RedTile {
    fn area_against(&self, other: &RedTile) -> u64 {
        (self.row.abs_diff(other.row) + 1) * (self.col.abs_diff(other.col) + 1)
    }
}

pub struct Day09;

impl Solution for Day09 {
    fn part1(&self, input: &str) -> String {
        let tiles = parser::array(parser::as_type::<u64>, ",")
            .map(|[row, col]| RedTile { row, col })
            .lines()
            .parse(input)
            .unwrap();

        tiles
            .iter()
            .tuple_combinations()
            .map(|(t1, t2)| t1.area_against(&t2))
            .max()
            .unwrap()
            .to_string()
    }

    fn part2(&self, input: &str) -> String {
        let tiles = parser::array(parser::as_type::<u64>, ",")
            .map(|[row, col]| RedTile { row, col })
            .lines()
            .parse(input)
            .unwrap();

        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::solutions::Solution;

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
        assert_eq!(Day09.part1(TEST_INPUT), "50");
    }

    #[test]
    fn test_part2() {
        assert_eq!(Day09.part2(TEST_INPUT), "25272");
    }
}
