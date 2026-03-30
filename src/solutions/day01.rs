use crate::solutions::Solution;
use crate::utils::parser;
use crate::utils::parser::{char_match, ParseError, Parser};

const DIAL_NUMBERS: i32 = 100;
const STARTING_NUMBER: i32 = 50;

#[derive(PartialEq)]
enum Direction {
    Left,
    Right,
}

struct DialAction {
    direction: Direction,
    distance: i32,
}

fn parse_actions(input: &str) -> Result<Vec<DialAction>, ParseError> {
    let parse_direction = char_match! {
        'L' => Direction::Left,
        'R' => Direction::Right,
    };
    parser::uncons(parse_direction, parser::as_type::<i32>)
        .map(|(direction, distance)| DialAction {
            direction,
            distance,
        })
        .lines()
        .parse(input)
}

pub struct Day01;

impl Solution for Day01 {
    fn part1(&self, input: &str) -> String {
        let actions = parse_actions(input).unwrap();
        let mut result = 0;
        let mut curr = STARTING_NUMBER;
        for action in actions {
            match action.direction {
                Direction::Right => curr += action.distance,
                Direction::Left => curr -= action.distance,
            }
            curr = curr.rem_euclid(DIAL_NUMBERS);

            if curr == 0 {
                result += 1;
            }
        }
        result.to_string()
    }

    fn part2(&self, input: &str) -> String {
        let actions = parse_actions(input).unwrap();
        let mut result = 0;
        let mut curr = STARTING_NUMBER;
        let mut was_zero = false;
        for action in actions {
            match action.direction {
                Direction::Right => curr += action.distance,
                Direction::Left => curr -= action.distance,
            }
            result += curr.div_euclid(DIAL_NUMBERS).abs();
            curr = curr.rem_euclid(DIAL_NUMBERS);

            if action.direction == Direction::Left {
                if was_zero {
                    result -= 1;
                }
                if curr == 0 {
                    result += 1;
                }
            }
            was_zero = curr == 0;
        }
        result.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::solutions::Solution;

    const TEST_INPUT: &str = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";

    #[test]
    fn test_part1() {
        assert_eq!(Day01.part1(TEST_INPUT), "3");
    }

    #[test]
    fn test_part2() {
        assert_eq!(Day01.part2(TEST_INPUT), "6");
    }
}
