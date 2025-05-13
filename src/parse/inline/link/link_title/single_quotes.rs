use crate::{
    inline::link::SingleQuotesLinkTitle,
    parse::{
        segment::link_title::{
            SingleQuotesLinkTitleMultiSegments, SingleQuotesLinkTitleSingleSegment,
        },
        traits::Parse,
    },
};
use nom::{IResult, Parser, branch::alt, error::ParseError};

impl<'a> Parse<'a> for SingleQuotesLinkTitle<'a> {
    fn parse<Error: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, Self, Error>
    where
        Self: Sized,
    {
        alt((
            SingleQuotesLinkTitleSingleSegment::parse.map(Self::Single),
            SingleQuotesLinkTitleMultiSegments::parse.map(Self::Multi),
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

        test_parse_macros!(SingleQuotesLinkTitle);

        failure_case!(should_reject_empty, "");
        failure_case!(should_reject_blank_line, " \t\n");

        success_case!(should_accept_single_segment, "'Hello!'\n", parsed => SingleQuotesLinkTitle::Single(SingleQuotesLinkTitleSingleSegment::strict_parse("'Hello!'")), "\n");
        success_case!(should_accept_multi_segments, "'Hello,\nWorld!'", parsed => SingleQuotesLinkTitle::Multi(SingleQuotesLinkTitleMultiSegments::strict_parse("'Hello,\nWorld!'")));
    }
}
