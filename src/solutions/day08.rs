use crate::solutions::Solution;
use crate::utils::geometry::{self, Point3};
use crate::utils::union_find::UnionFind;
use itertools::Itertools;

const NUM_CONNECTIONS_PART_1: usize = 1000;

pub struct Day08;

impl Day08 {
    fn largest_3_connection_groups_product(input: &str, num_connections: usize) -> usize {
        let points = input
            .lines()
            .map(|l| l.parse::<Point3<f64>>().unwrap())
            .collect_vec();

        let pairs = geometry::k_closest_pair_indices(&points, num_connections);
        let mut union_find = UnionFind::new(points.len());
        for (left, right) in pairs {
            union_find.union(left, right);
        }

        (0..points.len())
            .map(|i| union_find.find(i))
            .unique()
            .collect::<Vec<_>>()
            .into_iter()
            .map(|root| union_find.get_size(root))
            .k_largest(3)
            .product::<usize>()
    }
}

impl Solution for Day08 {
    fn part1(&self, input: &str) -> String {
        Day08::largest_3_connection_groups_product(input, NUM_CONNECTIONS_PART_1).to_string()
    }

    fn part2(&self, input: &str) -> String {
        let points = input
            .lines()
            .map(|l| l.parse::<Point3<f64>>().unwrap())
            .collect_vec();

        let pairs = geometry::closest_pair_indices(&points);
        let mut union_find = UnionFind::new(points.len());

        for (left, right) in pairs {
            union_find.union(left, right);
            if union_find.get_size(left) == points.len() {
                return (points[left].x() * points[right].x()).to_string();
            }
        }
        "COULD NOT FIND ANSWER".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::solutions::Solution;

    const TEST_INPUT: &str = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";

    #[test]
    fn test_part1() {
        assert_eq!(
            Day08::largest_3_connection_groups_product(TEST_INPUT, 10),
            40
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(Day08.part2(TEST_INPUT), "25272");
    }
}
