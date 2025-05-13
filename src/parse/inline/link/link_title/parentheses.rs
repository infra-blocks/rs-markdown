use crate::{
    inline::link::ParenthesesLinkTitle,
    parse::{
        segment::link_title::{
            ParenthesesLinkTitleMultiSegments, ParenthesesLinkTitleSingleSegment,
        },
        traits::Parse,
    },
};
use nom::{IResult, Parser, branch::alt, error::ParseError};

impl<'a> Parse<'a> for ParenthesesLinkTitle<'a> {
    fn parse<Error: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, Self, Error>
    where
        Self: Sized,
    {
        alt((
            ParenthesesLinkTitleSingleSegment::parse.map(Self::Single),
            ParenthesesLinkTitleMultiSegments::parse.map(Self::Multi),
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

        test_parse_macros!(ParenthesesLinkTitle);

        failure_case!(should_reject_empty, "");
        failure_case!(should_reject_blank_line, " \t\n");

        success_case!(should_accept_single_segment, "(Hello!)\n", parsed => ParenthesesLinkTitle::Single(ParenthesesLinkTitleSingleSegment::strict_parse("(Hello!)")), "\n");
        success_case!(should_accept_multi_segments, "(Hello,\nWorld!)", parsed => ParenthesesLinkTitle::Multi(ParenthesesLinkTitleMultiSegments::strict_parse("(Hello,\nWorld!)")));
    }
}
