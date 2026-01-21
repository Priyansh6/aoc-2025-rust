use crate::solutions::Solution;
use crate::utils::range::Range;
use itertools::Itertools;
use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
struct RedTile {
    row: u64,
    col: u64,
}

#[derive(Debug, Clone, Copy)]
struct Rectangle {
    min_row: u64,
    min_col: u64,
    max_row: u64,
    max_col: u64,
}

#[derive(Debug, Clone, Copy)]
struct VerticalWall {
    col: u64,
    row_range: Range<u64>,
}

#[derive(Debug, Clone, Copy)]
struct HorizontalWall {
    row: u64,
    col_range: Range<u64>,
}

impl RedTile {
    fn area_against(&self, other: &RedTile) -> u64 {
        (self.row.abs_diff(other.row) + 1) * (self.col.abs_diff(other.col) + 1)
    }
}

impl FromStr for RedTile {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut vals = s.split(',');
        let col = vals
            .next()
            .ok_or(format!("Could not extract range from string: {}", s))?
            .parse()
            .map_err(|e| format!("Could not parse range, {}", e))?;
        let row = vals
            .next()
            .ok_or(format!("Could not extract range from string: {}", s))?
            .parse()
            .map_err(|e| format!("Could not parse range, {}", e))?;

        Ok(RedTile { row, col })
    }
}

pub struct Day09;

impl Solution for Day09 {
    fn part1(&self, input: &str) -> String {
        let tiles = input
            .lines()
            .map(|line| line.parse::<RedTile>().unwrap())
            .collect_vec();

        tiles
            .iter()
            .tuple_combinations()
            .map(|(t1, t2)| t1.area_against(&t2))
            .max()
            .unwrap()
            .to_string()
    }

    fn part2(&self, input: &str) -> String {
        let mut tiles = input
            .lines()
            .map(|line| line.parse::<RedTile>().unwrap())
            .collect_vec();

        let mut vertical_walls = Vec::new();
        let mut horizontal_walls = Vec::new();
        for (t1, t2) in tiles.iter().circular_tuple_windows() {
            if t1.row == t2.row {
                horizontal_walls.push(HorizontalWall {
                    row: t1.row,
                    col_range: Range {
                        start: t1.col,
                        end: t2.col,
                    },
                });
            }
            if t1.col == t2.col {
                vertical_walls.push(VerticalWall {
                    col: t1.col,
                    row_range: Range {
                        start: t1.row,
                        end: t2.row,
                    },
                });
            }
        }
        vertical_walls.sort_by_key(|wall| wall.col);
        horizontal_walls.sort_by_key(|wall| wall.row);
        let vertical_walls = vertical_walls
            .chunk_by(|w1, w2| w1.col == w2.col && w1.row_range.overlaps_with(&w2.row_range))
            .map(|chunk| {
                chunk.into_iter().cloned().reduce(|r1, r2| VerticalWall {
                    col: r1.col,
                    row_range: r1.row_range.merged_with(r2.row_range),
                })
            })
            .filter(Option::is_some);

        for wall in vertical_walls {}
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
