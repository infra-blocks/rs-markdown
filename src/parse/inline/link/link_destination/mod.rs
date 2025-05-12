mod bracketed;
mod unbracketed;

use crate::inline::link::{BracketedLinkDestination, LinkDestination, UnbracketedLinkDestination};
use crate::parse::traits::Parse;
use nom::{IResult, Parser, branch::alt, error::ParseError};

impl<'a> Parse<'a> for LinkDestination<'a> {
    fn parse<Error: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, Self, Error>
    where
        Self: Sized,
    {
        alt((
            BracketedLinkDestination::parse.map(Self::from),
            UnbracketedLinkDestination::parse.map(Self::from),
        ))
        .parse(input)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    mod parse {
        use super::*;
        use crate::parse::{test_utils::test_parse_macros, traits::StrictParse};

        test_parse_macros!(LinkDestination);

        failure_case!(should_reject_empty_segment, "");
        failure_case!(should_reject_blank_line, "\n");

        success_case!(
            should_work_with_a_bracketed_variant,
            "<bracketed>",
            parsed => LinkDestination::Bracketed(BracketedLinkDestination::strict_parse("<bracketed>"))
        );
        success_case!(
            should_work_with_an_unbracketed_variant,
            "unbracketed",
            parsed => LinkDestination::Unbracketed(UnbracketedLinkDestination::strict_parse("unbracketed"))
        );
    }
}
