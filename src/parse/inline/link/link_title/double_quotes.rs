use crate::{
    inline::link::DoubleQuotesLinkTitle,
    parse::{
        segment::link_title::{
            DoubleQuotesLinkTitleMultiSegments, DoubleQuotesLinkTitleSingleSegment,
        },
        traits::Parse,
    },
};
use nom::{IResult, Parser, branch::alt, error::ParseError};

impl<'a> Parse<'a> for DoubleQuotesLinkTitle<'a> {
    fn parse<Error: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, Self, Error>
    where
        Self: Sized,
    {
        alt((
            DoubleQuotesLinkTitleSingleSegment::parse.map(Self::Single),
            DoubleQuotesLinkTitleMultiSegments::parse.map(Self::Multi),
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

        test_parse_macros!(DoubleQuotesLinkTitle);

        failure_case!(should_reject_empty, "");
        failure_case!(should_reject_blank_line, " \t\n");

        success_case!(should_accept_single_segment, "\"Hello!\"\n", parsed => DoubleQuotesLinkTitle::Single(DoubleQuotesLinkTitleSingleSegment::strict_parse("\"Hello!\"")), "\n");
        success_case!(should_accept_multi_segments, "\"Hello,\nWorld!\"", parsed => DoubleQuotesLinkTitle::Multi(DoubleQuotesLinkTitleMultiSegments::strict_parse("\"Hello,\nWorld!\"")));
    }
}
