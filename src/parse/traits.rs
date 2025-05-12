use nom::{
    IResult,
    error::{Error, ParseError},
};
use std::fmt::Debug;

/// The trait formalizing the parsing interface of structs.
///
/// It is a thin wrapper around [nom]'s parsing semantics.
pub trait Parse<'a> {
    fn parse<Error: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, Self, Error>
    where
        Self: Sized;
}

/// The error type returned by [ParseWhole::parse_whole].
///
/// It can either be a [nom::Err] in the case of a parsing error, or a [ParseWholeError::RemainingInput] variant
/// in the case of partial input consumption.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseWholeError<'a, E: ParseError<&'a str>> {
    RemainingInput(&'a str),
    NomError(nom::Err<E>),
}

/// This trait extends the [Parse] trait to provide a way to guarantee that the entire input has been consumed
/// on success, otherwise returning an error.
///
/// A blanket implementation is provided for all types that implement [Parse].
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

/// Yet another utility trait on top of [Parse] that provides a way to parse a struct
/// using [ParseWhole] and unwrap the result.
///
/// This is expected to be mostly useful in the context of tests. The error used by
/// the blanket implementation is [nom::error::Error].
pub trait StrictParse<'a> {
    #[allow(dead_code)]
    fn strict_parse(input: &'a str) -> Self
    where
        Self: Sized;
}

impl<'a, T> StrictParse<'a> for T
where
    T: ParseWhole<'a>,
{
    fn strict_parse(input: &'a str) -> Self {
        Self::parse_whole::<Error<&str>>(input).unwrap()
    }
}
