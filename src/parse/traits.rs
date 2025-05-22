use super::input::Input;
use nom::{
    IResult,
    error::{Error, ParseError},
};
use parser::{Enumerator, ParseResult};

/// The trait formalizing the parsing interface of structs.
///
/// It is a thin wrapper around [nom]'s parsing semantics.
pub trait NomParse<'a>
where
    Self: Sized,
{
    fn nom_parse<Error: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, Self, Error>;
}

/// For types that can be parsed from a single line of input.
pub trait ParseLine<'a>
where
    Self: Sized,
{
    fn parse_line(input: &'a str) -> ParseResult<&'a str, Self>;
}

impl<'a, T> ParseLine<'a> for T
where
    T: NomParse<'a>,
{
    fn parse_line(input: &'a str) -> ParseResult<&'a str, Self> {
        match Self::nom_parse::<Error<&str>>(input) {
            Ok((remaining, parsed)) => Ok((remaining, parsed)),
            Err(_) => Err(input),
        }
    }
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
    fn parse<I: Input<T>>(input: I) -> ParseResult<I, Self>;
}

impl<'a, T> Parse<&'a str> for T
where
    T: ParseLine<'a>,
{
    fn parse<I: Input<&'a str>>(input: I) -> ParseResult<I, Self> {
        let mut enumerator = input.enumerate();
        let Some((_, item)) = enumerator.next() else {
            std::mem::drop(enumerator);
            return Err(input);
        };
        match Self::parse_line(item) {
            Ok((remaining, parsed)) => {
                if remaining.is_empty() {
                    let (_, remaining) = input.split_at(enumerator.next_index());
                    Ok((remaining, parsed))
                } else {
                    let (_, remaining) = input.split_at(input.subset_range(remaining).0);
                    Ok((remaining, parsed))
                }
            }
            Err(_) => {
                std::mem::drop(enumerator);
                Err(input)
            }
        }
    }
}
