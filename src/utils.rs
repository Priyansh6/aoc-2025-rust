pub mod grid;
pub mod range;

use std::fmt;
use std::str;

pub fn parse_lines<T>(input: &str) -> impl Iterator<Item = T> + '_
where
    T: str::FromStr,
    T::Err: fmt::Debug,
{
    input.lines().map(|l| l.parse().unwrap())
}

pub fn digits_to_num(digits: &[u32]) -> u64 {
    let mut result: u64 = 0;
    let mut unit: u64 = 1;
    for &digit in digits.iter().rev() {
        result += unit * (digit as u64);
        unit *= 10;
    }
    result
}

pub fn row_to_column_major<T>(rows: Vec<Vec<T>>) -> Vec<Vec<T>> {
    if rows.is_empty() {
        return Vec::new();
    }
    let num_cols = rows[0].len();
    let mut row_iters: Vec<_> = rows.into_iter().map(|row| row.into_iter()).collect();

    (0..num_cols)
        .map(|_| {
            row_iters
                .iter_mut()
                .filter_map(|iter| iter.next())
                .collect()
        })
        .collect()
}
