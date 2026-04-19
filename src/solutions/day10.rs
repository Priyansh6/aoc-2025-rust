use crate::char_match;
use crate::solutions::Solution;
use crate::utils::parser;
use crate::utils::parser::{lsplit_once, rsplit_once, CharParser, Parser, StrParser};

pub struct Sol;

impl Solution for Sol {
    type Parsed = Vec<(Vec<bool>, Vec<Vec<usize>>, Vec<usize>)>;

    fn parser(&self) -> impl Parser<&str, Output = Self::Parsed> {
        let indicator = char_match! {
            '.' => false,
            '#' => true,
        };
        let indicator_parser = indicator.chars().wrapped("[", "]");
        let schematic_parser = parser::from_str::<usize>
            .split(",")
            .wrapped("(", ")")
            .split(" ");
        let requirement_parser = parser::from_str::<usize>.split(",").wrapped("{", "}");
        lsplit_once(
            indicator_parser,
            rsplit_once(schematic_parser, requirement_parser, " "),
            " ",
        )
        .map(|(indicators, (schematics, requirements))| (indicators, schematics, requirements))
        .lines()
    }

    fn part1(&self, machines: &Self::Parsed) -> String {
        todo!()
    }

    fn part2(&self, machines: &Self::Parsed) -> String {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::solutions::{check_part1, check_part2};

    const TEST_INPUT: &str = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";

    #[test]
    fn test_part1() {
        check_part1(&crate::solutions::day09::Sol, TEST_INPUT, "50");
    }

    #[test]
    fn test_part2() {
        check_part2(&crate::solutions::day09::Sol, TEST_INPUT, "24");
    }
}
