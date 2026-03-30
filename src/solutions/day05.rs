use crate::solutions::Solution;
use crate::utils::parser;
use crate::utils::parser::Parser;
use crate::utils::range::Range;

type IdType = u64;

pub struct Day05;

impl Solution for Day05 {
    fn part1(&self, input: &str) -> String {
        let range_parser = parser::as_type::<Range<IdType>>.lines();
        let id_parser = parser::as_type::<IdType>.lines();
        let (ranges, ids) = parser::pair(range_parser, id_parser, "\n\n")
            .parse(input)
            .unwrap();
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
        let range_parser = parser::as_type::<Range<IdType>>.lines();
        let (mut ranges, _) = parser::pair(range_parser, parser::unit, "\n\n")
            .parse(input)
            .unwrap();
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
