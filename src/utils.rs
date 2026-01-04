use std::fmt;
use std::str;

pub fn parse_lines<T>(input: &str) -> impl Iterator<Item = T> + '_
where
    T: str::FromStr,
    T::Err: fmt::Debug,
{
    input.lines().map(|l| l.parse().unwrap())
}
