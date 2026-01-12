use crate::solutions;
use crate::utils::range::Range;
use std::str;

type IdType = u64;

fn is_id_invalid_part1(id: &IdType) -> bool {
    let digits = id.ilog10() + 1;
    if digits % 2 == 1 {
        return false;
    };

    let id_str = id.to_string();
    let (first_half, second_half) = id_str.split_at(id_str.len() / 2);
    first_half == second_half
}

fn is_id_invalid_part2(id: &IdType) -> bool {
    let digits = id.ilog10() + 1;

    let id_str = id.to_string();
    let chars: Vec<char> = id_str.chars().collect();
    for size in (1..digits).filter(|&size| digits % size == 0) {
        let chunks: Vec<String> = chars
            .chunks(size as usize)
            .map(|chunk| chunk.iter().collect())
            .collect();
        if chunks.windows(2).all(|w| w[0] == w[1]) {
            return true;
        }
    }
    false
}

pub struct Day02;

impl solutions::Solution for Day02 {
    fn part1(&self, input: &str) -> String {
        let ranges = input
            .split(",")
            .map(|r| r.parse::<Range<IdType>>().unwrap());
        let mut result = 0;
        for range in ranges {
            result += range.iter().filter(is_id_invalid_part1).sum::<IdType>();
        }
        result.to_string()
    }

    fn part2(&self, input: &str) -> String {
        let ranges = input
            .split(",")
            .map(|r| r.parse::<Range<IdType>>().unwrap());
        let mut result = 0;
        for range in ranges {
            result += range.iter().filter(is_id_invalid_part2).sum::<IdType>();
        }
        result.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::solutions::Solution;

    const TEST_INPUT: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

    #[test]
    fn test_part1() {
        assert_eq!(Day02.part1(TEST_INPUT), "1227775554");
    }

    #[test]
    fn test_part2() {
        assert_eq!(Day02.part2(TEST_INPUT), "4174379265");
    }
}
