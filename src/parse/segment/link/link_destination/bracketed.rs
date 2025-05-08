use crate::parse::{
    traits::{Parse, Segment},
    utils::escaped_sequence,
};
use nom::{
    IResult, Parser, branch::alt, bytes::complete::is_not, character::complete::char,
    combinator::recognize, error::ParseError, multi::many0,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BracketedLinkDestinationSegment<'a>(&'a str);

impl<'a> BracketedLinkDestinationSegment<'a> {
    fn new(segment: &'a str) -> Self {
        Self(segment)
    }
}

impl<'a> Parse<'a> for BracketedLinkDestinationSegment<'a> {
    fn parse<Error: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, Self, Error>
    where
        Self: Sized,
    {
        recognize((
            char('<'),
            many0(alt((escaped_sequence, is_not("\n\\<>")))),
            char('>'),
        ))
        .map(Self::new)
        .parse(input)
    }
}

impl<'a> Segment<'a> for BracketedLinkDestinationSegment<'a> {
    fn segment(&self) -> &'a str {
        self.0
    }
}

#[cfg(test)]
mod test {
    use super::*;

    mod parse {
        use super::*;
        use nom::error::Error;

        macro_rules! failure_case {
            ($test:ident, $segment:expr) => {
                #[test]
                fn $test() {
                    assert!(
                        BracketedLinkDestinationSegment::parse::<Error<&str>>($segment).is_err()
                    );
                }
            };
        }

        macro_rules! success_case {
            ($test:ident, $segment:expr) => {
                #[test]
                fn $test() {
                    assert_eq!(
                        BracketedLinkDestinationSegment::parse::<Error<&str>>($segment),
                        Ok(("", BracketedLinkDestinationSegment::new($segment)))
                    );
                }
            };
        }

        failure_case!(should_reject_empty_segment, "");
        failure_case!(should_reject_newline, "\n");
        failure_case!(should_reject_single_opening_bracket, "<");
        failure_case!(should_reject_missing_unescaped_closing_bracket, r"<\>");
        failure_case!(should_reject_duplicate_opening_bracket, r"<<>");

        success_case!(should_work_with_empty_brackets, "<>");
        success_case!(should_work_with_a_parenthesis, "<)>");
        success_case!(should_work_with_several_parentheses, "<()(()))>");
        success_case!(should_work_with_any_characters, "<hellö>");
    }
}
