#![allow(dead_code)]

pub mod error;
pub use error::ParseError;
use std::fmt::Display;
use std::str::FromStr;

pub trait Parser<T> {
    fn parse(&self, input: &str) -> Result<T, ParseError>;

    fn map<U>(self, f: impl Fn(T) -> U) -> impl Parser<U>
    where
        Self: Sized,
    {
        move |input: &str| self.parse(input).map(|v| f(v))
    }

    fn and_then<U>(self, f: impl Fn(T) -> Result<U, ParseError>) -> impl Parser<U>
    where
        Self: Sized,
    {
        move |input: &str| self.parse(input).and_then(|v| f(v))
    }

    fn lines(self) -> impl Parser<Vec<T>>
    where
        Self: Sized,
    {
        move |input: &str| input.lines().map(|l| self.parse(l)).collect()
    }

    fn split_whitespace(self) -> impl Parser<Vec<T>>
    where
        Self: Sized,
    {
        move |input: &str| input.split_whitespace().map(|v| self.parse(v)).collect()
    }
}

// Parser blanket impl
impl<T, F: Fn(&str) -> Result<T, ParseError>> Parser<T> for F {
    fn parse(&self, input: &str) -> Result<T, ParseError> {
        self(input)
    }
}

pub trait CharParser<T> {
    fn parse(&self, input: char) -> Result<T, ParseError>;

    fn into_parser(self) -> impl Parser<T>
    where
        Self: Sized,
    {
        move |input: &str| {
            let mut chars = input.chars();
            let c = chars.next().ok_or(ParseError::EmptyInput)?;
            if chars.next().is_some() {
                return Err(ParseError::WrongLength {
                    expected: 1,
                    got: input.chars().count(),
                    input: input.to_string(),
                });
            }
            self.parse(c)
        }
    }

    fn char_list(self) -> impl Parser<Vec<T>>
    where
        Self: Sized,
    {
        move |input: &str| input.chars().map(|c| self.parse(c)).collect()
    }
}

// CharParser blanket impl
impl<T, F: Fn(char) -> Result<T, ParseError>> CharParser<T> for F {
    fn parse(&self, input: char) -> Result<T, ParseError> {
        self(input)
    }
}

/// Creates a `CharParser` closure that maps specific characters to values.
///
/// Returns an error for any character not in the mapping.
///
/// # Example
/// ```
/// let parse_square = char_match! {
///     '.' => Square::Blank,
///     '@' => Square::Paper,
/// };
/// ```
macro_rules! char_match {
    ($($c:literal => $val:expr),+ $(,)?) => {
        |c: char| match c {
            $($c => Ok($val),)+
            _ => Err($crate::utils::parser::ParseError::Other(
                format!("Unexpected character: '{c}'")
            ))
        }
    };
}
pub(crate) use char_match;

pub fn as_string(s: &str) -> Result<String, ParseError> {
    Ok(s.to_string())
}

pub fn char_identity(c: char) -> Result<char, ParseError> {
    Ok(c)
}

pub fn unit(_s: &str) -> Result<(), ParseError> {
    Ok(())
}

pub fn as_type<T: FromStr>(s: &str) -> Result<T, ParseError>
where
    T::Err: Display,
{
    s.parse::<T>().map_err(|e| e.to_string().into())
}

pub fn char_as<T: FromStr>(c: char) -> Result<T, ParseError>
where
    T::Err: Display,
{
    // We avoid a heap allocation by not converting to string, and instead using the stack
    c.encode_utf8(&mut [0; 4])
        .parse::<T>()
        .map_err(|e| e.to_string().into())
}

pub fn digit<const RADIX: u32>(c: char) -> Result<u32, ParseError> {
    c.to_digit(RADIX).ok_or(ParseError::NotADigit(c))
}

pub fn array<T, const N: usize>(element: impl Parser<T>, separator: &str) -> impl Parser<[T; N]> {
    move |input: &str| {
        input
            .split(separator)
            .map(|v| element.parse(v))
            .collect::<Result<Vec<_>, _>>()?
            .try_into()
            .map_err(|v: Vec<T>| ParseError::WrongLength {
                expected: N,
                got: v.len(),
                input: input.to_string(),
            })
    }
}

pub fn list<T>(element: impl Parser<T>, separator: &str) -> impl Parser<Vec<T>> {
    move |input: &str| input.split(separator).map(|v| element.parse(v)).collect()
}

pub fn pair<T, U>(
    left: impl Parser<T>,
    right: impl Parser<U>,
    separator: &str,
) -> impl Parser<(T, U)> {
    move |input: &str| {
        let elems: Vec<&str> = input.split(separator).collect();
        match elems.as_slice() {
            [l, r] => Ok((left.parse(l)?, right.parse(r)?)),
            _ => Err(ParseError::WrongLength {
                expected: 2,
                got: elems.len(),
                input: input.to_string(),
            }),
        }
    }
}

pub fn uncons<T, U>(first: impl CharParser<T>, rest: impl Parser<U>) -> impl Parser<(T, U)> {
    move |input: &str| {
        let mut chars = input.chars();
        let c = chars.next().ok_or(ParseError::EmptyInput)?;
        let a = first.parse(c)?;
        let b = rest.parse(chars.as_str())?;
        Ok((a, b))
    }
}

pub fn rsplit_once<T, U>(
    body: impl Parser<T>,
    last: impl Parser<U>,
    separator: &str,
) -> impl Parser<(T, U)> {
    move |input: &str| {
        let (rest, last_line) = input.rsplit_once(separator).ok_or(ParseError::EmptyInput)?;
        Ok((body.parse(rest)?, last.parse(last_line)?))
    }
}
