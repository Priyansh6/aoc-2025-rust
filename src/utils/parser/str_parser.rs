use crate::utils::parser::{ParseError, Parser};

pub trait StrParser: for<'a> Parser<&'a str> {
    fn lines(self) -> Lines<Self>
    where
        Self: Sized,
    {
        Lines { parser: self }
    }

    fn split(self, separator: &str) -> Split<Self>
    where
        Self: Sized,
    {
        Split {
            parser: self,
            separator: separator.to_string(),
        }
    }

    fn split_whitespace(self) -> SplitWhitespace<Self>
    where
        Self: Sized,
    {
        SplitWhitespace { parser: self }
    }

    fn split_array<const N: usize>(self, separator: &str) -> SplitArray<Self, N>
    where
        Self: Sized,
    {
        SplitArray {
            parser: self,
            separator: separator.to_string(),
        }
    }
}

// Blanket impl for functions
impl<P> StrParser for P where P: for<'a> Parser<&'a str> {}

pub struct Lines<P> {
    parser: P,
}

impl<P, T> Parser<&str> for Lines<P>
where
    P: for<'a> Parser<&'a str, Output = T>,
{
    type Output = Vec<T>;

    fn parse(&self, input: &str) -> Result<Self::Output, ParseError> {
        input.lines().map(|l| self.parser.parse(l)).collect()
    }
}

pub struct Split<P> {
    parser: P,
    separator: String,
}

impl<P, T> Parser<&str> for Split<P>
where
    P: for<'a> Parser<&'a str, Output = T>,
{
    type Output = Vec<T>;

    fn parse(&self, input: &str) -> Result<Self::Output, ParseError> {
        input
            .split(&self.separator)
            .map(|v| self.parser.parse(v))
            .collect()
    }
}

pub struct SplitWhitespace<P> {
    parser: P,
}

impl<P, T> Parser<&str> for SplitWhitespace<P>
where
    P: for<'a> Parser<&'a str, Output = T>,
{
    type Output = Vec<T>;

    fn parse(&self, input: &str) -> Result<Vec<T>, ParseError> {
        input
            .split_whitespace()
            .map(|v| self.parser.parse(v))
            .collect()
    }
}

pub struct SplitArray<P, const N: usize> {
    parser: P,
    separator: String,
}

impl<T, P, const N: usize> Parser<&str> for SplitArray<P, N>
where
    P: for<'a> Parser<&'a str, Output = T>,
{
    type Output = [T; N];

    fn parse(&self, input: &str) -> Result<Self::Output, ParseError> {
        input
            .split(&self.separator)
            .map(|v| self.parser.parse(v))
            .collect::<Result<Vec<_>, _>>()?
            .try_into()
            .map_err(|v: Vec<T>| ParseError::WrongLength {
                expected: N,
                got: v.len(),
                input: input.to_string(),
            })
    }
}
