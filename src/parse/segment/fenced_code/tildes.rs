use crate::{
    Segment,
    parse::{
        parsers::{indented_by_less_than_4, line_ending_or_empty, space_or_tab},
        traits::ParseLine,
    },
};
use parser::{Map, ParseResult, Parser, consumed};

pub fn tildes_fenced_code_opening_segment<'a>(
    input: &'a str,
) -> ParseResult<&'a str, TildesFencedCodeOpeningSegment<'a>> {
    consumed((
        indented_by_less_than_4,
        utils::tildes_fence,
        utils::info_string,
    ))
    .map(
        |(segment, (indent, fence, info_string)): (&'a str, (&'a str, &'a str, &'a str))| {
            TildesFencedCodeOpeningSegment::new(segment, indent.len(), fence.len(), info_string)
        },
    )
    .parse(input)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TildesFencedCodeOpeningSegment<'a> {
    segment: &'a str,
    indent: usize,
    // The amount of tildes used, minimally 3.
    fence_length: usize,
    info_string: &'a str,
}

impl<'a> TildesFencedCodeOpeningSegment<'a> {
    fn new(segment: &'a str, indent: usize, fence_length: usize, info_string: &'a str) -> Self {
        Self {
            segment,
            indent,
            fence_length,
            info_string,
        }
    }
}

impl<'a> ParseLine<'a> for TildesFencedCodeOpeningSegment<'a> {
    fn parse_line(input: &'a str) -> ParseResult<&'a str, Self> {
        tildes_fenced_code_opening_segment(input)
    }
}

impl<'a> Segment<'a> for TildesFencedCodeOpeningSegment<'a> {
    fn segment(&self) -> &'a str {
        self.segment
    }
}

pub fn tildes_fenced_code_closing_segment<'a>(
    input: &'a str,
) -> ParseResult<&'a str, TildesFencedCodeClosingSegment<'a>> {
    consumed((
        indented_by_less_than_4,
        utils::tildes_fence,
        space_or_tab(),
        line_ending_or_empty,
    ))
    .map(
        |(segment, (indent, fence, _, _)): (&'a str, (&'a str, &'a str, &'a str, &'a str))| {
            TildesFencedCodeClosingSegment::new(segment, indent.len(), fence.len())
        },
    )
    .parse(input)
}

// Closing segments don't have info strings.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TildesFencedCodeClosingSegment<'a> {
    segment: &'a str,
    indent: usize,
    fence_length: usize,
}

impl<'a> TildesFencedCodeClosingSegment<'a> {
    /// Returns true if the closing segment is a valid closure for the opening segment.
    ///
    /// This is only true if the closing segment's fence is at least as long as the opening segment's fence.
    pub fn closes(&self, opening: &TildesFencedCodeOpeningSegment) -> bool {
        self.fence_length >= opening.fence_length
    }

    fn new(segment: &'a str, indent: usize, fence_length: usize) -> Self {
        Self {
            segment,
            indent,
            fence_length,
        }
    }
}

impl<'a> ParseLine<'a> for TildesFencedCodeClosingSegment<'a> {
    fn parse_line(input: &'a str) -> ParseResult<&'a str, Self> {
        tildes_fenced_code_closing_segment(input)
    }
}

impl<'a> Segment<'a> for TildesFencedCodeClosingSegment<'a> {
    fn segment(&self) -> &'a str {
        self.segment
    }
}

#[cfg(test)]
mod test {
    use super::*;

    mod opening {
        use super::*;
        use crate::parse::test_utils::test_parse_macros;

        test_parse_macros!(TildesFencedCodeOpeningSegment);

        failure_case!(should_reject_empy, "");
        failure_case!(should_reject_blank_line, "\n");
        failure_case!(should_reject_2_tildes, "~~\n");
        failure_case!(should_reject_4_whitespace_indent, "    ~~~\n");
        failure_case!(should_reject_tab_indent, "\t~~~\n");

        success_case!(
            should_work_with_3_tildes,
            "~~~\n",
            parsed => TildesFencedCodeOpeningSegment::new("~~~\n", 0, 3, "")
        );
        success_case!(
            should_work_without_trailing_newline,
            "~~~",
            parsed => TildesFencedCodeOpeningSegment::new("~~~", 0, 3, "")
        );
        success_case!(
            should_work_with_3_tildes_and_3_whitespace_ident,
            "   ~~~\n",
            parsed => TildesFencedCodeOpeningSegment::new("   ~~~\n", 3, 3, "")
        );
        success_case!(
            should_work_with_info_string,
            "~~~rust\n",
            parsed => TildesFencedCodeOpeningSegment::new("~~~rust\n", 0, 3, "rust")
        );
        success_case!(
            should_work_with_info_string_without_trailing_newline,
            "~~~rust",
            parsed => TildesFencedCodeOpeningSegment::new("~~~rust", 0, 3, "rust")
        );
        success_case!(
            should_work_tildes_in_info_string,
            "~~~rust~\n",
            parsed => TildesFencedCodeOpeningSegment::new("~~~rust~\n", 0, 3, "rust~")
        );
        success_case!(
            should_work_backticks_in_info_string,
            "~~~rust`\n",
            parsed => TildesFencedCodeOpeningSegment::new("~~~rust`\n", 0, 3, "rust`")
        );
        success_case!(
            should_work_with_padded_info_string,
            "~~~   rust is kind of fucking cool   \n",
            parsed => TildesFencedCodeOpeningSegment::new(
                "~~~   rust is kind of fucking cool   \n",
                0,
                3,
                "rust is kind of fucking cool"
            )
        );
    }

    mod closing {
        use super::*;
        use crate::parse::test_utils::test_parse_macros;

        test_parse_macros!(TildesFencedCodeClosingSegment);

        failure_case!(should_reject_empy, "");
        failure_case!(should_reject_blank_line, "\n");
        failure_case!(should_reject_2_tildes, "~~\n");
        failure_case!(should_reject_info_string, "~~~rust\n");
        failure_case!(should_reject_4_whitespace_indent, "    ~~~\n");
        failure_case!(should_reject_tab_indent, "\t~~~\n");

        success_case!(
            should_work_with_3_tildes,
            "~~~\n",
            parsed => TildesFencedCodeClosingSegment::new("~~~\n", 0, 3)
        );
        success_case!(
            should_work_without_trailing_newline,
            "~~~",
            parsed => TildesFencedCodeClosingSegment::new("~~~", 0, 3)
        );
        success_case!(
            should_work_with_4_tildes,
            "~~~~\n",
            parsed => TildesFencedCodeClosingSegment::new("~~~~\n", 0, 4)
        );
        success_case!(
            should_work_with_trailing_whitespaces,
            "~~~   \t\n",
            parsed => TildesFencedCodeClosingSegment::new("~~~   \t\n", 0, 3)
        );
        success_case!(
            should_work_with_3_whitespaces_indent,
            "   ~~~\n",
            parsed => TildesFencedCodeClosingSegment::new("   ~~~\n", 3, 3)
        );
    }
}

mod utils {
    use parser::{Map, ParseResult, Parser, equals, rest, take_while};

    pub fn tildes_fence(input: &str) -> ParseResult<&str, &str> {
        take_while(equals('~')).at_least(3).parse(input)
    }

    pub fn info_string<'a>(input: &'a str) -> ParseResult<&'a str, &'a str> {
        rest.map(|parsed: &'a str| parsed.trim()).parse(input)
    }

    #[cfg(test)]
    mod test {
        use super::*;

        mod tildes_fence {
            use super::*;

            macro_rules! failure_case {
                ($test:ident, $segment:expr) => {
                    #[test]
                    fn $test() {
                        assert!(tildes_fence($segment).is_err());
                    }
                };
            }

            macro_rules! success_case {
                ($test:ident, $segment:expr, $expected:expr) => {
                    #[test]
                    fn $test() {
                        assert_eq!(tildes_fence($segment), Ok(("", $expected)));
                    }
                };
            }

            failure_case!(should_reject_empty, "");
            failure_case!(should_reject_1_backtick, "~");
            failure_case!(should_reject_2_tildes, "~~");

            success_case!(should_work_with_3_tildes, "~~~", "~~~");
            success_case!(should_work_with_4_tildes, "~~~~", "~~~~");
            success_case!(should_work_with_5_tildes, "~~~~~", "~~~~~");
        }
    }
}
