use super::{
    ParseResult,
    input::{Input, ParseQuantity},
};
use nom::{
    IResult,
    error::{Error, ParseError},
};

/// The trait formalizing the parsing interface of structs.
///
/// It is a thin wrapper around [nom]'s parsing semantics.
pub trait NomParse<'a>
where
    Self: Sized,
{
    fn nom_parse<Error: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, Self, Error>;
}

/// This trait is the main interface for parsing.
///
/// The implementer is expected to receive the input and return a [ParseResult].
/// [ParseResult]s can be obtained from the [Input].
pub trait Parse<T>
where
    Self: Sized,
{
    /// Parse the input and return the remaining input and the parsed value.
    fn parse<I: Input<Item = T>>(input: I) -> ParseResult<I, Self>;
}

impl<'a, T> Parse<&'a str> for T
where
    T: NomParse<'a>,
{
    fn parse<I: Input<Item = &'a str>>(input: I) -> ParseResult<I, Self> {
        let segment = input.first().unwrap_or_default();
        match Self::nom_parse::<Error<&str>>(segment) {
            Ok((remaining, parsed)) => {
                if remaining.is_empty() {
                    input.parsed(ParseQuantity::Items(1), parsed)
                } else {
                    let bytes_remaining = remaining.len();
                    let bytes_consumed = segment.len() - bytes_remaining;
                    input.parsed(ParseQuantity::Bytes(bytes_consumed), parsed)
                }
            }
            Err(_) => input.failed(),
        }
    }
}
