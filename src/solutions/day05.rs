use crate::solutions;
use crate::utils::range::Range;
use std::str;

type IdType = u64;

pub struct Day05;

impl solutions::Solution for Day05 {
    fn part1(&self, input: &str) -> String {
        let mut input = input.split("\n\n").collect::<Vec<&str>>().into_iter();
        let ranges: Vec<Range<IdType>> = input
            .next()
            .unwrap()
            .lines()
            .map(|line| line.parse().unwrap())
            .collect();
        let ids: Vec<IdType> = input
            .next()
            .unwrap()
            .lines()
            .map(|line| line.parse().unwrap())
            .collect();
        let mut fresh_ids = 0;
        for &id in &ids {
            for range in &ranges {
                if range.contains(id) {
                    fresh_ids += 1;
                    break;
                }
            }
        }
        fresh_ids.to_string()
    }

    fn part2(&self, input: &str) -> String {
        let mut ranges: Vec<Range<IdType>> = input
            .split("\n\n")
            .next()
            .unwrap()
            .lines()
            .map(|line| line.parse().unwrap())
            .collect();
        ranges.sort_by_key(|range| range.start);

        let mut merged_ranges: Vec<Range<IdType>> = Vec::new();

        for range in ranges {
            if let Some(last) = merged_ranges.last_mut()
                && last.overlaps_with(&range)
            {
                last.merge(range);
            } else {
                merged_ranges.push(range);
            }
        }

        merged_ranges
            .iter()
            .map(|range| range.num_elems())
            .sum::<usize>()
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::solutions::Solution;

    const TEST_INPUT: &str = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";

    #[test]
    fn test_part1() {
        assert_eq!(Day05.part1(TEST_INPUT), "3");
    }

    #[test]
    fn test_part2() {
        assert_eq!(Day05.part2(TEST_INPUT), "14");
    }
}
