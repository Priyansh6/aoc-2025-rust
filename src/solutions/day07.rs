use crate::solutions;
use crate::utils::grid::{Grid, GridPosition};
use std::collections::{HashMap, HashSet};
use std::str;

#[derive(PartialEq, Copy, Clone)]
enum Square {
    Blank,
    Source,
    Splitter,
    Beam,
}

impl str::FromStr for Square {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 1 {
            return Err(format!("Invalid square format: \"{}\"", s));
        }
        match s.chars().next().unwrap() {
            '.' => Ok(Square::Blank),
            'S' => Ok(Square::Source),
            '^' => Ok(Square::Splitter),
            '|' => Ok(Square::Beam),
            _ => Err(format!("Invalid square character: \"{}\"", s)),
        }
    }
}

fn insert_beam(beams: &mut HashMap<GridPosition, u64>, pos: GridPosition, possibilities: u64) {
    beams
        .entry(pos)
        .and_modify(|p| *p += possibilities)
        .or_insert(possibilities);
}

pub struct Day07;

impl solutions::Solution for Day07 {
    fn part1(&self, input: &str) -> String {
        let manifold: Grid<Square> = input.parse().unwrap();
        let source_pos = manifold.find(&Square::Source).unwrap();
        let mut beam_pos = HashSet::from([source_pos]);
        let mut collisions = 0;
        for _ in 0..(manifold.height() - 1) {
            let mut next_beam_pos: HashSet<GridPosition> = HashSet::new();
            for beam in beam_pos {
                let below_pos = manifold.below(&beam).unwrap();
                let next = manifold[below_pos];
                match next {
                    Square::Blank => {
                        next_beam_pos.insert(below_pos);
                    }
                    Square::Splitter => {
                        collisions += 1;
                        if let Some(left) = manifold.left(&below_pos) {
                            next_beam_pos.insert(left);
                        }
                        if let Some(right) = manifold.right(&below_pos) {
                            next_beam_pos.insert(right);
                        }
                    }
                    Square::Beam => panic!("There should not be beams on the grid"),
                    Square::Source => panic!("There should only be one source on the grid"),
                }
            }
            beam_pos = next_beam_pos;
        }
        collisions.to_string()
    }

    fn part2(&self, input: &str) -> String {
        let manifold: Grid<Square> = input.parse().unwrap();
        let source_pos = manifold.find(&Square::Source).unwrap();
        let mut beam_possibilities: HashMap<GridPosition, u64> = HashMap::from([(source_pos, 1)]);
        for _ in 0..(manifold.height() - 1) {
            let mut next_beam_possibilities: HashMap<GridPosition, u64> = HashMap::new();
            for (pos, possibilities) in beam_possibilities {
                let below_pos = manifold.below(&pos).unwrap();
                let next = manifold[below_pos];
                match next {
                    Square::Blank => {
                        insert_beam(&mut next_beam_possibilities, below_pos, possibilities)
                    }
                    Square::Splitter => {
                        if let Some(left) = manifold.left(&below_pos) {
                            insert_beam(&mut next_beam_possibilities, left, possibilities);
                        }
                        if let Some(right) = manifold.right(&below_pos) {
                            insert_beam(&mut next_beam_possibilities, right, possibilities);
                        }
                    }
                    Square::Beam => panic!("There should not be beams on the grid"),
                    Square::Source => panic!("There should only be one source on the grid"),
                }
            }
            beam_possibilities = next_beam_possibilities;
        }
        beam_possibilities.values().sum::<u64>().to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::solutions::Solution;

    const TEST_INPUT: &str = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";

    #[test]
    fn test_part1() {
        assert_eq!(Day07.part1(TEST_INPUT), "21");
    }

    #[test]
    fn test_part2() {
        assert_eq!(Day07.part2(TEST_INPUT), "40");
    }
}
