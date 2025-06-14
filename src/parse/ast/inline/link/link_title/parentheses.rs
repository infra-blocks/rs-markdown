use crate::{
    ast::inline::link::ParenthesesLinkTitle,
    parse::{
        input::Input,
        segment::link_title::{
            ParenthesesLinkTitleMultiSegments, ParenthesesLinkTitleSingleSegment,
        },
        traits::Parse,
    },
};
use parser::{Map, ParseResult, Parser, one_of};

impl<'a> Parse<'a> for ParenthesesLinkTitle<'a> {
    fn parse<I: Input<'a>>(input: I) -> ParseResult<I, Self> {
        one_of((
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
        use crate::parse::test_utils::{StrictParse, test_parse_macros};

        test_parse_macros!(ParenthesesLinkTitle);

        failure_case!(should_reject_empty, "");
        failure_case!(should_reject_blank_line, " \t\n");

        success_case!(should_accept_single_segment, "(Hello!)\n", parsed => ParenthesesLinkTitle::Single(ParenthesesLinkTitleSingleSegment::strict_parse("(Hello!)")), "\n");
        success_case!(should_accept_multi_segments, "(Hello,\nWorld!)", parsed => ParenthesesLinkTitle::Multi(ParenthesesLinkTitleMultiSegments::strict_parse("(Hello,\nWorld!)")));
    }
}
