use crate::{
    ast::inline::link::SingleQuotesLinkTitle,
    parse::{
        input::Input,
        segment::link_title::{
            SingleQuotesLinkTitleMultiSegments, SingleQuotesLinkTitleSingleSegment,
        },
        traits::Parse,
    },
};
use parser::{Map, ParseResult, Parser, one_of};

impl<'a> Parse<'a> for SingleQuotesLinkTitle<'a> {
    fn parse<I: Input<'a>>(input: I) -> ParseResult<I, Self> {
        one_of((
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
        use crate::parse::test_utils::{StrictParse, test_parse_macros};

        test_parse_macros!(SingleQuotesLinkTitle);

        failure_case!(should_reject_empty, "");
        failure_case!(should_reject_blank_line, " \t\n");

        success_case!(should_accept_single_segment, "'Hello!'\n", parsed => SingleQuotesLinkTitle::Single(SingleQuotesLinkTitleSingleSegment::strict_parse("'Hello!'")), "\n");
        success_case!(should_accept_multi_segments, "'Hello,\nWorld!'", parsed => SingleQuotesLinkTitle::Multi(SingleQuotesLinkTitleMultiSegments::strict_parse("'Hello,\nWorld!'")));
    }
}
