use crate::solutions;
use crate::utils;
use std::cmp;

fn propagate_max_and_set_next_to_zero(slice: &mut [u32], val: u32) {
    let len = slice.len();
    for i in 0..len - 1 {
        if val > slice[i] {
            slice[i] = val;
            slice[i + 1] = 0;
            return;
        }
    }
    slice[len - 1] = cmp::max(val, slice[len - 1]);
}

fn solution(input: &str, num_batteries: usize) -> u64 {
    let digit_lines = input.lines().map(|line| {
        line.chars()
            .map(|c| c.to_digit(10).unwrap())
            .collect::<Vec<u32>>()
    });

    let mut sum: u64 = 0;
    for digits in digit_lines {
        let mut result_digits = vec![0; num_batteries];
        for (i, &digit) in digits.iter().enumerate() {
            propagate_max_and_set_next_to_zero(
                &mut result_digits[num_batteries.saturating_sub(digits.len() - i)..],
                digit,
            );
        }
        sum += utils::digits_to_num(result_digits.as_slice());
    }
    sum
}

pub struct Day03;

impl solutions::Solution for Day03 {
    fn part1(&self, input: &str) -> String {
        solution(input, 2).to_string()
    }

    fn part2(&self, input: &str) -> String {
        solution(input, 12).to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::solutions::Solution;

    const TEST_INPUT: &str = "987654321111111
811111111111119
234234234234278
818181911112111";

    #[test]
    fn test_part1() {
        assert_eq!(Day03.part1(TEST_INPUT), "357");
    }

    #[test]
    fn test_part2() {
        assert_eq!(Day03.part2(TEST_INPUT), "3121910778619");
    }

    #[test]
    fn test_part1_single_line() {
        assert_eq!(Day03.part1("987654321111111"), "98");
    }

    #[test]
    fn test_part2_single_line() {
        assert_eq!(Day03.part2("987654321111111"), "987654321111");
    }
}
