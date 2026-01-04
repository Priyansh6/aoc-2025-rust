use crate::solutions;
use std::str;

type IdType = u64;

struct Range {
    start: IdType,
    end: IdType,
}

impl Range {
    fn invalid_ids_part1(&self) -> impl Iterator<Item = IdType> + '_ {
        (self.start..=self.end).filter(Self::is_id_invalid_part1)
    }

    fn is_id_invalid_part1(id: &IdType) -> bool {
        let digits = id.ilog10() + 1;
        if digits % 2 == 1 {
            return false;
        };

        let id_str = id.to_string();
        let (first_half, second_half) = id_str.split_at(id_str.len() / 2);
        first_half == second_half
    }

    fn invalid_ids_part2(&self) -> impl Iterator<Item = IdType> + '_ {
        (self.start..=self.end).filter(Self::is_id_invalid_part2)
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
}

impl str::FromStr for Range {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut vals = s.split('-');
        let start = vals
            .next()
            .ok_or(format!("Could not extract range from string: {}", s))?
            .parse::<IdType>()
            .map_err(|e| format!("Could not parse range, {}", e))?;
        let end = vals
            .next()
            .ok_or(format!("Could not extract range from string: {}", s))?
            .parse::<IdType>()
            .map_err(|e| format!("Could not parse range, {}", e))?;

        Ok(Range { start, end })
    }
}

pub struct Day02;

impl solutions::Solution for Day02 {
    fn part1(&self, input: &str) -> String {
        let ranges = input.split(",").map(|r| r.parse::<Range>().unwrap());
        let mut result = 0;
        for range in ranges {
            result += range.invalid_ids_part1().sum::<IdType>();
        }
        result.to_string()
    }

    fn part2(&self, input: &str) -> String {
        let ranges = input.split(",").map(|r| r.parse::<Range>().unwrap());
        let mut result = 0;
        for range in ranges {
            result += range.invalid_ids_part2().sum::<IdType>();
        }
        result.to_string()
    }
}
