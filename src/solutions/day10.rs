use crate::solutions::Solution;
use crate::utils::algebra::GaussianEliminationGF2Result;
use crate::utils::parser::{lsplit_once, rsplit_once, CharParser, Parser, StrParser};
use crate::utils::{algebra, parser};
use crate::{char_match, utils};
use itertools::Itertools;

pub struct Sol;

fn get_possible_presses_for_indicators(
    schematics: &Vec<Vec<usize>>,
    indicators: &Vec<bool>,
) -> impl Iterator<Item = Vec<usize>> {
    let num_indicators = indicators.len();

    let mut schematics = schematics
        .iter()
        .map(|schematic| {
            let mut schematic_vector = vec![false; num_indicators];
            for &i in schematic {
                schematic_vector[i] = true;
            }
            schematic_vector
        })
        .collect_vec();
    schematics.push(indicators.clone());
    let matrix = utils::row_to_column_major(schematics);

    let GaussianEliminationGF2Result {
        reduced_matrix,
        pivot_cols,
        free_cols,
    } = algebra::gaussian_elimination_gf2(matrix);

    let aug_col = reduced_matrix[0].len() - 1;

    itertools::repeat_n([false, true], free_cols.len())
        .multi_cartesian_product()
        .map(move |free_vals| {
            let free_press_cols = free_cols
                .iter()
                .zip_eq(&free_vals)
                .filter(|&(_, &pressed)| pressed)
                .map(|(&free_col, _)| free_col);
            let pivot_press_cols = pivot_cols
                .iter()
                .enumerate()
                .filter(|&(row, _)| {
                    free_cols.iter().enumerate().fold(
                        reduced_matrix[row][aug_col],
                        |val, (free_col_i, &free_col)| {
                            val ^ (free_vals[free_col_i] && reduced_matrix[row][free_col])
                        },
                    )
                })
                .map(|(_, &pivot_col)| pivot_col);
            free_press_cols.chain(pivot_press_cols).collect_vec()
        })
}

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
        let mut sum_presses = 0;
        for (indicators, schematics, _) in machines {
            sum_presses += get_possible_presses_for_indicators(&schematics, &indicators)
                .map(|presses| presses.len())
                .min()
                .unwrap();
        }
        sum_presses.to_string()
    }

    fn part2(&self, machines: &Self::Parsed) -> String {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::solutions::{check_part1, check_part2};

    const TEST_INPUT: &str = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";

    #[test]
    fn test_part1() {
        check_part1(&crate::solutions::day10::Sol, TEST_INPUT, "7");
    }

    #[test]
    fn test_part2() {
        check_part2(&crate::solutions::day10::Sol, TEST_INPUT, "33");
    }
}
