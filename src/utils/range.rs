#![allow(dead_code)]

use crate::utils::parser;
use crate::utils::parser::{ParseError, Parser, StrParser};
use std::cmp;
use std::cmp::Ordering;
use std::error::Error;
use std::fmt::Display;
use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
pub struct Range<T> {
    start: T,
    end: T,
}

#[derive(Debug)]
pub struct RangeError;

impl Display for RangeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Range start must be <= end")
    }
}

impl Error for RangeError {}

impl Range<u64> {
    pub fn iter(&self) -> impl Iterator<Item = u64> {
        self.start..=self.end
    }

    pub fn num_elems(&self) -> usize {
        (self.end - self.start + 1) as usize
    }
}

impl<T: PartialOrd> Range<T> {
    pub fn new(start: T, end: T) -> Result<Self, RangeError> {
        if start <= end {
            Ok(Range { start, end })
        } else {
            Err(RangeError)
        }
    }

    pub fn between(a: T, b: T) -> Self {
        if a.partial_cmp(&b).unwrap() != Ordering::Greater {
            Range { start: a, end: b }
        } else {
            Range { start: b, end: a }
        }
    }

    pub fn start(&self) -> &T {
        &self.start
    }

    pub fn end(&self) -> &T {
        &self.end
    }

    pub fn contains(&self, x: &T) -> bool {
        &self.start <= x && x <= &self.end
    }

    pub fn contains_exclusive(&self, x: &T) -> bool {
        &self.start < x && x < &self.end
    }

    pub fn overlaps(&self, range: &Range<T>) -> bool {
        self.start <= range.end && range.start <= self.end
    }

    pub fn overlaps_strictly(&self, range: &Range<T>) -> bool {
        self.start < range.end && range.start < self.end
    }
}

impl<T: Ord + Copy> Range<T> {
    pub fn merge(&mut self, range: Range<T>) {
        self.start = cmp::min(self.start, range.start);
        self.end = cmp::max(self.end, range.end);
    }

    pub fn merged_with(mut self, range: Range<T>) -> Self {
        self.start = cmp::min(self.start, range.start);
        self.end = cmp::max(self.end, range.end);
        self
    }
}

impl<T> FromStr for Range<T>
where
    T: PartialOrd,
    T: FromStr,
    T::Err: Display,
{
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parser::from_str
            .split_array("-")
            .and_then(|[start, end]| {
                Range::new(start, end).map_err(|err| ParseError::Other(err.to_string()))
            })
            .parse(s)
    }
}
