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
