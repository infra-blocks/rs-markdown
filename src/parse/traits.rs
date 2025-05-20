use super::{
    input::Input,
    parser::{Enumerator, ParseResult},
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
    fn parse<I: Input<T>>(input: I) -> ParseResult<I, Self>;
}

impl<'a, T> Parse<&'a str> for T
where
    T: NomParse<'a>,
{
    fn parse<I: Input<&'a str>>(input: I) -> ParseResult<I, Self> {
        let mut enumerator = input.enumerate();
        let Some((_, item)) = enumerator.next() else {
            std::mem::drop(enumerator);
            return Err(input);
        };
        match Self::nom_parse::<Error<&str>>(item) {
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
