#![allow(dead_code)]

mod char_parser;
mod error;
mod str_parser;

pub use char_parser::CharParser;
pub use error::ParseError;
use std::fmt::Display;
use std::str::FromStr;
pub use str_parser::StrParser;

// === Core Trait ===

pub trait Parser<I> {
    type Output;

    fn parse(&self, input: I) -> Result<Self::Output, ParseError>;

    fn map<F, U>(self, f: F) -> Map<Self, F>
    where
        Self: Sized,
        F: Fn(Self::Output) -> U,
    {
        Map { parser: self, f }
    }

    fn and_then<F, U>(self, f: F) -> AndThen<Self, F>
    where
        Self: Sized,
        F: Fn(Self::Output) -> Result<U, ParseError>,
    {
        AndThen { parser: self, f }
    }

    fn into_each(self) -> IntoEach<Self>
    where
        Self: Sized,
    {
        IntoEach { parser: self }
    }
}

// Blanket impl for functions
impl<I, T, F> Parser<I> for F
where
    F: Fn(I) -> Result<T, ParseError>,
{
    type Output = T;

    fn parse(&self, input: I) -> Result<Self::Output, ParseError> {
        self(input)
    }
}

pub struct Map<P, F> {
    parser: P,
    f: F,
}

impl<I, U, P, F> Parser<I> for Map<P, F>
where
    P: Parser<I>,
    F: Fn(P::Output) -> U,
{
    type Output = U;

    fn parse(&self, input: I) -> Result<Self::Output, ParseError> {
        self.parser.parse(input).map(|v| (self.f)(v))
    }
}

pub struct AndThen<P, F> {
    parser: P,
    f: F,
}

impl<I, U, P, F> Parser<I> for AndThen<P, F>
where
    P: Parser<I>,
    F: Fn(P::Output) -> Result<U, ParseError>,
{
    type Output = U;

    fn parse(&self, input: I) -> Result<Self::Output, ParseError> {
        self.parser.parse(input).and_then(|v| (self.f)(v))
    }
}

pub struct IntoEach<P> {
    parser: P,
}

impl<P, C> Parser<C> for IntoEach<P>
where
    P: Parser<C::Item>,
    C: IntoIterator,
{
    type Output = Vec<P::Output>;

    fn parse(&self, input: C) -> Result<Self::Output, ParseError> {
        input
            .into_iter()
            .map(|item| self.parser.parse(item))
            .collect()
    }
}

// === Standalone parsers ===

pub fn as_str(s: &str) -> Result<&str, ParseError> {
    Ok(s)
}

pub fn as_string(s: &str) -> Result<String, ParseError> {
    Ok(s.to_string())
}

pub fn identity<T>(item: T) -> Result<T, ParseError> {
    Ok(item)
}

pub fn unit(_s: &str) -> Result<(), ParseError> {
    Ok(())
}

pub fn from_str<T>(s: &str) -> Result<T, ParseError>
where
    T: FromStr,
    T::Err: Display,
{
    s.parse::<T>().map_err(|e| e.to_string().into())
}

pub fn digit<const RADIX: u32>(c: char) -> Result<u32, ParseError> {
    c.to_digit(RADIX).ok_or(ParseError::NotADigit(c))
}

// === Standalone combinators ===

pub fn split_pair<T, U>(
    left: impl for<'a> Parser<&'a str, Output = T>,
    right: impl for<'a> Parser<&'a str, Output = U>,
    separator: &str,
) -> impl for<'a> Parser<&'a str, Output = (T, U)> {
    let separator = separator.to_string();
    move |input: &str| {
        let elems: Vec<&str> = input.split(&*separator).collect();
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

pub fn uncons<T, U>(
    first: impl CharParser<Output = T>,
    rest: impl for<'a> Parser<&'a str, Output = U>,
) -> impl for<'a> Parser<&'a str, Output = (T, U)> {
    move |input: &str| {
        let mut chars = input.chars();
        let c = chars.next().ok_or(ParseError::EmptyInput)?;
        let a = first.parse(c)?;
        let b = rest.parse(chars.as_str())?;
        Ok((a, b))
    }
}

pub fn rsplit_once<T, U>(
    body: impl for<'a> Parser<&'a str, Output = T>,
    last: impl for<'a> Parser<&'a str, Output = U>,
    separator: &str,
) -> impl for<'a> Parser<&'a str, Output = (T, U)> {
    let separator = separator.to_string();
    move |input: &str| {
        let (rest, last_line) = input
            .rsplit_once(&*separator)
            .ok_or(ParseError::EmptyInput)?;
        Ok((body.parse(rest)?, last.parse(last_line)?))
    }
}
