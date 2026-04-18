use crate::utils::parser::{ParseError, Parser};

pub trait CharParser: Parser<char> {
    fn single_char(self) -> SingleChar<Self>
    where
        Self: Sized,
    {
        SingleChar { parser: self }
    }

    fn chars(self) -> Chars<Self>
    where
        Self: Sized,
    {
        Chars { parser: self }
    }
}

// Blanket impl for functions
impl<T, F: Fn(char) -> Result<T, ParseError>> CharParser for F {}

pub struct SingleChar<P> {
    parser: P,
}

impl<P> Parser<&str> for SingleChar<P>
where
    P: Parser<char>,
{
    type Output = P::Output;

    fn parse(&self, input: &str) -> Result<Self::Output, ParseError> {
        let mut chars = input.chars();
        let c = chars.next().ok_or(ParseError::EmptyInput)?;
        if chars.next().is_some() {
            return Err(ParseError::WrongLength {
                expected: 1,
                got: input.chars().count(),
                input: input.to_string(),
            });
        }
        self.parser.parse(c)
    }
}

pub struct Chars<P> {
    parser: P,
}

impl<P: Parser<char>> Parser<&str> for Chars<P> {
    type Output = Vec<P::Output>;

    fn parse(&self, input: &str) -> Result<Self::Output, ParseError> {
        input.chars().map(|c| self.parser.parse(c)).collect()
    }
}

#[macro_export]
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
