use super::input::Input;
use parser::{Enumerator, ParseResult};

/// For types that can be parsed from a single line of input.
pub trait ParseLine<'a>
where
    Self: Sized,
{
    fn parse_line(input: &'a str) -> ParseResult<&'a str, Self>;
}

// TODO: parse_extend that receives an [Extend] implementer to stuff in the results yo.

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
