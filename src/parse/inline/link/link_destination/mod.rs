mod bracketed;
mod unbracketed;

use crate::inline::link::{BracketedLinkDestination, LinkDestination, UnbracketedLinkDestination};
use crate::parse::traits::NomParse;
use nom::{IResult, Parser, branch::alt, error::ParseError};

impl<'a> NomParse<'a> for LinkDestination<'a> {
    fn nom_parse<Error: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, Self, Error>
    where
        Self: Sized,
    {
        alt((
            BracketedLinkDestination::nom_parse.map(Self::from),
            UnbracketedLinkDestination::nom_parse.map(Self::from),
        ))
        .parse(input)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    mod parse {
        use super::*;
        use crate::parse::test_utils::{StrictParse, test_parse_macros};

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
