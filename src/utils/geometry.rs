#![allow(dead_code)]

use itertools::Itertools;
use std::str::FromStr;

pub type Vector3<T> = Vector<T, 3>;
pub struct Vector<T, const N: usize> {
    vals: [T; N],
}
pub type Point<T, const N: usize> = Vector<T, N>;
pub type Point3<T> = Vector3<T>;

impl<T, const N: usize> Vector<T, N> {
    pub fn new(vals: [T; N]) -> Self {
        Self { vals }
    }
}

impl FromStr for Vector3<f64> {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.split(',')
            .map(|d| d.trim().parse::<f64>())
            .try_collect::<_, Vec<_>, _>()
            .map_err(|e| e.to_string())?
            .try_into()
            .map(Vector3::new)
            .map_err(|v: Vec<_>| format!("Expected 3 values, got {}", v.len()))
    }
}

impl<T, const N: usize> std::ops::Index<usize> for Vector<T, N> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.vals[index]
    }
}

macro_rules! impl_accessors {
    ($n:literal, $($method:ident => $index:literal),+ $(,)?) => {
        impl<T: Copy> Vector<T, $n> {
            $(
                pub fn $method(&self) -> T {
                    self.vals[$index]
                }
            )+
        }
    };
}

impl_accessors!(1, x => 0);
impl_accessors!(2, x => 0, y => 1);
impl_accessors!(3, x => 0, y => 1, z => 2);

impl<const N: usize> Point<f64, N> {
    fn distance_from(&self, other: &Point<f64, N>) -> f64 {
        self.vals
            .iter()
            .zip_eq(other.vals.iter())
            .map(|(a, b)| (a - b).powi(2))
            .sum::<f64>()
            .sqrt()
    }
}

pub fn k_closest_pair_indices<const N: usize>(
    points: &Vec<Point<f64, N>>,
    k: usize,
) -> impl Iterator<Item = (usize, usize)> {
    points
        .iter()
        .enumerate()
        .tuple_combinations()
        .map(|((l_i, l_point), (r_i, r_point))| ((l_i, r_i), l_point.distance_from(r_point)))
        .k_smallest_relaxed_by(k, |(_, dist1), (_, dist2)| dist1.total_cmp(dist2))
        .map(|(pair, _)| pair)
}

pub fn closest_pair_indices<const N: usize>(
    points: &Vec<Point<f64, N>>,
) -> impl Iterator<Item = (usize, usize)> {
    points
        .iter()
        .enumerate()
        .tuple_combinations()
        .map(|((l_i, l_point), (r_i, r_point))| ((l_i, r_i), l_point.distance_from(r_point)))
        .sorted_by(|(_, dist1), (_, dist2)| dist1.total_cmp(dist2))
        .map(|(pair, _)| pair)
}
