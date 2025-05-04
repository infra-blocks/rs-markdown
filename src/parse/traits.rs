use std::fmt::Debug;

use nom::{error::ParseError, IResult};

pub trait Parse<'a> {
    fn parse<Error: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, Self, Error>
    where
        Self: Sized;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseWholeError<'a, E: ParseError<&'a str>> {
    RemainingInput(&'a str),
    NomError(nom::Err<E>),
}

pub trait ParseWhole<'a> {
    fn parse_whole<Error: ParseError<&'a str>>(
        input: &'a str,
    ) -> Result<Self, ParseWholeError<'a, Error>>
    where
        Self: Sized;
}

impl<'a, T> ParseWhole<'a> for T
where
    T: Parse<'a>,
{
    fn parse_whole<Error: ParseError<&'a str>>(
        input: &'a str,
    ) -> Result<Self, ParseWholeError<'a, Error>> {
        match Self::parse(input) {
            Ok((remaining, result)) => {
                if remaining.is_empty() {
                    Ok(result)
                } else {
                    Err(ParseWholeError::RemainingInput(remaining))
                }
            }
            Err(err) => Err(ParseWholeError::NomError(err)),
        }
    }
}
