use std::cmp;
use std::fmt::Display;
use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
pub struct Range<T> {
    pub start: T,
    pub end: T,
}

impl Range<u64> {
    pub fn iter(&self) -> impl Iterator<Item = u64> {
        self.start..=self.end
    }

    pub fn num_elems(&self) -> usize {
        (self.end - self.start + 1) as usize
    }
}

impl<T: PartialOrd> Range<T> {
    pub fn contains(&self, x: T) -> bool {
        self.start <= x && x <= self.end
    }

    pub fn overlaps_with(&self, range: &Range<T>) -> bool {
        (self.start <= range.start && range.start <= self.end)
            || (self.end <= range.end && range.end <= self.end)
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
    T: FromStr,
    T::Err: Display,
{
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut vals = s.split('-');
        let start = vals
            .next()
            .ok_or(format!("Could not extract range from string: {}", s))?
            .parse::<T>()
            .map_err(|e| format!("Could not parse range, {}", e))?;
        let end = vals
            .next()
            .ok_or(format!("Could not extract range from string: {}", s))?
            .parse::<T>()
            .map_err(|e| format!("Could not parse range, {}", e))?;

        Ok(Range { start, end })
    }
}
