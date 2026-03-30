use crate::solutions::Solution;
use crate::utils;
use crate::utils::parser;
use crate::utils::parser::{char_match, CharParser, ParseError, Parser};
use itertools::Itertools;

enum Operator {
    Add,
    Multiply,
}

fn parse_operator(c: char) -> Result<Operator, ParseError> {
    (char_match! {
        '+' => Operator::Add,
        '*' => Operator::Multiply,
    })(c)
}

fn calculate_sum(operators: &[Operator], num_groups: &[Vec<u64>]) -> u64 {
    operators
        .iter()
        .zip_eq(num_groups)
        .map(|(op, nums)| match op {
            Operator::Add => nums.iter().sum::<u64>(),
            Operator::Multiply => nums.iter().product::<u64>(),
        })
        .sum()
}

pub struct Day06;

impl Solution for Day06 {
    fn part1(&self, input: &str) -> String {
        let num_grid_parser = parser::as_type::<u64>.split_whitespace().lines();
        let operators_parser = parse_operator.into_parser().split_whitespace();
        let (num_grid, operators) = parser::rsplit_once(num_grid_parser, operators_parser, "\n")
            .parse(input)
            .unwrap();

        let num_groups = utils::row_to_column_major(num_grid);
        calculate_sum(&operators, &num_groups).to_string()
    }

    fn part2(&self, input: &str) -> String {
        let char_grid_parser = parser::identity.char_list().lines();
        let operators_parser = parse_operator.into_parser().split_whitespace();
        let (char_grid, operators) = parser::rsplit_once(char_grid_parser, operators_parser, "\n")
            .parse(input)
            .unwrap();

        // Convert grid to column-major format as characters
        let col_major_char_grid: Vec<Vec<char>> = utils::row_to_column_major(char_grid);

        // Group columns by empty spaces to form number groups
        let num_groups: Vec<Vec<u64>> = col_major_char_grid
            .iter()
            .map(|col| col.iter().collect::<String>().trim().to_string())
            .batching(|it| {
                // Take consecutive non-empty strings as one group
                let nums = it
                    .take_while(|s| !s.is_empty())
                    .map(|s| s.parse().unwrap())
                    .collect_vec();
                (!nums.is_empty()).then_some(nums)
            })
            .collect();

        calculate_sum(&operators, &num_groups).to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::solutions::Solution;

    const TEST_INPUT: &str = concat!(
        "123 328  51 64 \n",
        " 45 64  387 23 \n",
        "  6 98  215 314\n",
        "*   +   *   +  "
    );

    #[test]
    fn test_part1() {
        assert_eq!(Day06.part1(TEST_INPUT), "4277556");
    }

    #[test]
    fn test_part2() {
        assert_eq!(Day06.part2(TEST_INPUT), "3263827");
    }
}
