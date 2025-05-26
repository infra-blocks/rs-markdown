use crate::{
    ast::inline::link::DoubleQuotesLinkTitle,
    parse::{
        input::Input,
        segment::link_title::{
            DoubleQuotesLinkTitleMultiSegments, DoubleQuotesLinkTitleSingleSegment,
        },
        traits::Parse,
    },
};
use parser::{Map, ParseResult, Parser, one_of};

impl<'a> Parse<'a> for DoubleQuotesLinkTitle<'a> {
    fn parse<I: Input<'a>>(input: I) -> ParseResult<I, Self> {
        one_of((
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
        use crate::parse::test_utils::{StrictParse, test_parse_macros};

        test_parse_macros!(DoubleQuotesLinkTitle);

        failure_case!(should_reject_empty, "");
        failure_case!(should_reject_blank_line, " \t\n");

        success_case!(should_accept_single_segment, "\"Hello!\"\n", parsed => DoubleQuotesLinkTitle::Single(DoubleQuotesLinkTitleSingleSegment::strict_parse("\"Hello!\"")), "\n");
        success_case!(should_accept_multi_segments, "\"Hello,\nWorld!\"", parsed => DoubleQuotesLinkTitle::Multi(DoubleQuotesLinkTitleMultiSegments::strict_parse("\"Hello,\nWorld!\"")));
    }
}
