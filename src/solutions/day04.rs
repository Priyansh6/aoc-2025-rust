use crate::solutions::Solution;
use crate::utils::grid::{Grid, GridPosition};
use std::str::FromStr;

#[derive(PartialEq, Copy, Clone)]
enum Square {
    Blank,
    Paper,
}

impl FromStr for Square {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 1 {
            return Err(format!("Invalid square format: \"{}\"", s));
        }
        match s.chars().next().unwrap() {
            '.' => Ok(Square::Blank),
            '@' => Ok(Square::Paper),
            _ => Err(format!("Invalid square character: \"{}\"", s)),
        }
    }
}

pub struct Day04;

impl Day04 {
    fn get_accessible_paper_positions(grid: &Grid<Square>) -> impl Iterator<Item = GridPosition> {
        grid.iter_enumerated().filter_map(|(pos, square)| {
            let is_accessible = *square == Square::Paper
                && grid
                    .surrounding_cells(pos)
                    .filter(|&cell| *cell == Square::Paper)
                    .count()
                    < 4;

            is_accessible.then_some(pos)
        })
    }
}

impl Solution for Day04 {
    fn part1(&self, input: &str) -> String {
        let grid: Grid<Square> = input.parse().unwrap();
        Day04::get_accessible_paper_positions(&grid)
            .count()
            .to_string()
    }

    fn part2(&self, input: &str) -> String {
        let mut grid: Grid<Square> = input.parse().unwrap();
        let mut total_accessible_squares = 0;
        while let accessible_square_positions =
            Day04::get_accessible_paper_positions(&mut grid).collect::<Vec<GridPosition>>()
            && accessible_square_positions.len() > 0
        {
            for &pos in &accessible_square_positions {
                grid[pos] = Square::Blank;
            }
            total_accessible_squares += accessible_square_positions.len();
        }
        total_accessible_squares.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::solutions::Solution;

    const TEST_INPUT: &str = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";

    #[test]
    fn test_part1() {
        assert_eq!(Day04.part1(TEST_INPUT), "13");
    }

    #[test]
    fn test_part2() {
        assert_eq!(Day04.part2(TEST_INPUT), "43");
    }
}
