use crate::utils::parser::{ParseError, Parser};

/// A [`Parser`] specialised for `&str` inputs, with combinators for splitting and
/// iterating over string data.
///
/// Any type implementing `Parser<&str>` automatically gains these combinators via the
/// blanket impl below.
pub trait StrParser: for<'a> Parser<&'a str> {
    /// Applies this parser to each line of the input, collecting results into a `Vec`.
    ///
    /// Lines are split by `\n` or `\r\n` (via [`str::lines`]). Fails fast on the first
    /// line that does not parse.
    ///
    /// # Example
    /// ```
    /// let p = from_str::<u32>.lines();
    /// assert_eq!(p.parse("1\n2\n3"), Ok(vec![1, 2, 3]));
    /// ```
    fn lines(self) -> Lines<Self>
    where
        Self: Sized,
    {
        Lines { parser: self }
    }

    /// Splits the input on `separator` and applies this parser to each part, collecting
    /// results into a `Vec`.
    ///
    /// Fails fast on the first part that does not parse.
    ///
    /// # Example
    /// ```
    /// let p = from_str::<u32>.split(",");
    /// assert_eq!(p.parse("1,2,3"), Ok(vec![1, 2, 3]));
    /// ```
    fn split(self, separator: &str) -> Split<Self>
    where
        Self: Sized,
    {
        Split {
            parser: self,
            separator: separator.to_string(),
        }
    }

    /// Splits the input on any whitespace and applies this parser to each token,
    /// collecting results into a `Vec`.
    ///
    /// Uses [`str::split_whitespace`], so leading, trailing, and consecutive whitespace
    /// are all handled gracefully. Fails fast on the first token that does not parse.
    ///
    /// # Example
    /// ```
    /// let p = from_str::<u32>.split_whitespace();
    /// assert_eq!(p.parse("1  2\t3"), Ok(vec![1, 2, 3]));
    /// ```
    fn split_whitespace(self) -> SplitWhitespace<Self>
    where
        Self: Sized,
    {
        SplitWhitespace { parser: self }
    }

    /// Splits the input on `separator` and applies this parser to each part, collecting
    /// results into a fixed-size array of length `N`.
    ///
    /// Returns [`ParseError::WrongLength`] if the number of parts is anything other
    /// than `N`. Fails fast on the first part that does not parse.
    ///
    /// # Example
    /// ```
    /// let p = from_str::<u32>.split_array::<3>(",");
    /// assert_eq!(p.parse("1,2,3"), Ok([1, 2, 3]));
    /// assert!(p.parse("1,2").is_err());     // too few
    /// assert!(p.parse("1,2,3,4").is_err()); // too many
    /// ```
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

/// Blanket [`StrParser`] implementation for any type that implements `Parser<&str>`.
///
/// This means all combinators on [`StrParser`] are available automatically, including
/// on closures and the combinator structs from the parent module.
impl<P> StrParser for P where P: for<'a> Parser<&'a str> {}

/// A `&str` parser that applies an inner [`StrParser`] to each line of the input.
///
/// Constructed via [`StrParser::lines`].
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

/// A `&str` parser that splits the input on a fixed separator and applies an inner
/// [`StrParser`] to each part.
///
/// Constructed via [`StrParser::split`].
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

/// A `&str` parser that splits the input on whitespace and applies an inner
/// [`StrParser`] to each token.
///
/// Constructed via [`StrParser::split_whitespace`].
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

/// A `&str` parser that splits the input on a fixed separator and applies an inner
/// [`StrParser`] to each part, collecting results into a fixed-size array of length `N`.
///
/// Constructed via [`StrParser::split_array`].
pub struct SplitArray<P, const N: usize> {
    parser: P,
    separator: String,
}

impl<T, P, const N: usize> Parser<&str> for SplitArray<P, N>
where
    P: for<'a> Parser<&'a str, Output = T>,
{
    type Output = [T; N];

    /// Splits `input` on the stored separator, parses each part, and converts the
    /// resulting `Vec` into a `[T; N]`.
    ///
    /// Returns [`ParseError::WrongLength`] if the split does not yield exactly `N` parts.
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
