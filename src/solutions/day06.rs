use crate::solutions;
use crate::utils;
use itertools::Itertools;
use std::str;

pub struct Day06;

enum Operator {
    Add,
    Multiply,
}

impl str::FromStr for Operator {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let chars: Vec<char> = s.chars().collect();
        if chars.len() != 1 {
            return Err(format!(
                "Could not parse {} as an Operator, it is not a single character",
                s
            ));
        }
        match chars[0] {
            '+' => Ok(Operator::Add),
            '*' => Ok(Operator::Multiply),
            _ => Err(format!("Unknown operator {}", s)),
        }
    }
}

impl solutions::Solution for Day06 {
    fn part1(&self, input: &str) -> String {
        let mut lines = input.lines();
        let mut operators: Vec<Operator> = Vec::new();
        if let Some(operators_line) = lines.next_back() {
            for operator in operators_line.split_whitespace() {
                operators.push(operator.parse().unwrap());
            }
        }
        let num_grid: Vec<Vec<u64>> = lines
            .map(|l| {
                l.split_whitespace()
                    .map(|num| num.parse().unwrap())
                    .collect()
            })
            .collect();
        let num_grid = utils::row_to_column_major(num_grid);
        let mut sum = 0;
        for (operator, nums) in operators.iter().zip_eq(&num_grid) {
            sum += match operator {
                Operator::Add => nums.iter().sum::<u64>(),
                Operator::Multiply => nums.iter().product::<u64>(),
            }
        }
        sum.to_string()
    }

    fn part2(&self, input: &str) -> String {
        let mut lines = input.lines();
        let mut operators: Vec<Operator> = Vec::new();
        if let Some(operators_line) = lines.next_back() {
            for operator in operators_line.split_whitespace() {
                operators.push(operator.parse().unwrap());
            }
        }
        let col_major_char_grid: Vec<Vec<char>> =
            utils::row_to_column_major(lines.map(|line| line.chars().collect()).collect());
        let problem_nums: Vec<Vec<u64>> = col_major_char_grid
            .iter()
            .map(|col| col.iter().collect::<String>().trim().to_string())
            .batching(|it| {
                let nums = it
                    .take_while(|s| !s.is_empty())
                    .map(|s| s.parse().unwrap())
                    .collect_vec();
                (!nums.is_empty()).then_some(nums)
            })
            .collect();

        let mut sum = 0;
        for (operator, nums) in operators.iter().zip_eq(&problem_nums) {
            sum += match operator {
                Operator::Add => nums.iter().sum::<u64>(),
                Operator::Multiply => nums.iter().product::<u64>(),
            }
        }
        sum.to_string()
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
