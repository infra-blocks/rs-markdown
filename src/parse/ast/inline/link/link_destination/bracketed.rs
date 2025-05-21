use crate::ast::inline::link::BracketedLinkDestination;
use crate::parse::{traits::NomParse, utils::escaped_sequence};
use nom::{
    IResult, Parser, branch::alt, bytes::complete::is_not, character::complete::char,
    combinator::recognize, error::ParseError, multi::many0,
};

impl<'a> NomParse<'a> for BracketedLinkDestination<'a> {
    fn nom_parse<Error: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, Self, Error>
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

#[cfg(test)]
mod test {
    use super::*;

    mod parse {
        use super::*;
        use crate::parse::test_utils::test_parse_macros;

        test_parse_macros!(BracketedLinkDestination);

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
