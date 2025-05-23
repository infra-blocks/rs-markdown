use crate::Segment;
use crate::parse::parser_utils::{indented_by_less_than_4, line_ending_or_eof, space_or_tab};
use crate::parse::traits::ParseLine;
use parser::{Map, ParseResult, Parser, consumed};

pub fn backticks_fenced_code_opening_segment<'a>(
    input: &'a str,
) -> ParseResult<&'a str, BackticksFencedCodeOpeningSegment<'a>> {
    consumed((
        indented_by_less_than_4,
        utils::backticks_fence,
        utils::info_string,
    ))
    .map(
        |(segment, (indent, fence, info_string)): (&'a str, (&'a str, &'a str, &'a str))| {
            BackticksFencedCodeOpeningSegment::new(segment, indent.len(), fence.len(), info_string)
        },
    )
    .parse(input)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BackticksFencedCodeOpeningSegment<'a> {
    segment: &'a str,
    indent: usize,
    // The amount of backticks used, minimally 3.
    fence_length: usize,
    info_string: &'a str,
}

impl<'a> BackticksFencedCodeOpeningSegment<'a> {
    fn new(segment: &'a str, indent: usize, fence_length: usize, info_string: &'a str) -> Self {
        Self {
            segment,
            indent,
            fence_length,
            info_string,
        }
    }
}

impl<'a> ParseLine<'a> for BackticksFencedCodeOpeningSegment<'a> {
    fn parse_line(input: &'a str) -> ParseResult<&'a str, Self> {
        backticks_fenced_code_opening_segment(input)
    }
}

impl<'a> Segment<'a> for BackticksFencedCodeOpeningSegment<'a> {
    fn segment(&self) -> &'a str {
        self.segment
    }
}

pub fn backticks_fenced_code_closing_segment<'a>(
    input: &'a str,
) -> ParseResult<&'a str, BackticksFencedCodeClosingSegment<'a>> {
    consumed((
        indented_by_less_than_4,
        utils::backticks_fence,
        space_or_tab,
        line_ending_or_eof,
    ))
    .map(
        |(segment, (indent, fence, _, _)): (&'a str, (&'a str, &'a str, &'a str, &'a str))| {
            BackticksFencedCodeClosingSegment::new(segment, indent.len(), fence.len())
        },
    )
    .parse(input)
}

// Closing segments don't have info strings.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BackticksFencedCodeClosingSegment<'a> {
    segment: &'a str,
    indent: usize,
    fence_length: usize,
}

impl<'a> BackticksFencedCodeClosingSegment<'a> {
    /// Returns true if the closing segment is a valid closure for the opening segment.
    ///
    /// This is only true if the closing segment's fence is at least as long as the opening segment's fence.
    pub fn closes(&self, opening: &BackticksFencedCodeOpeningSegment) -> bool {
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

impl<'a> ParseLine<'a> for BackticksFencedCodeClosingSegment<'a> {
    fn parse_line(input: &'a str) -> ParseResult<&'a str, Self> {
        backticks_fenced_code_closing_segment(input)
    }
}

impl<'a> Segment<'a> for BackticksFencedCodeClosingSegment<'a> {
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

        test_parse_macros!(BackticksFencedCodeOpeningSegment);

        failure_case!(should_reject_empy, "");
        failure_case!(should_reject_blank_line, "\n");
        failure_case!(should_reject_2_backticks, "``\n");
        failure_case!(should_reject_backticks_in_info_string, "```rust`\n");
        failure_case!(should_reject_4_whitespace_indent, "    ```\n");
        failure_case!(should_reject_tab_indent, "\t```\n");

        success_case!(
            should_work_with_3_backticks,
            "```\n",
            parsed => BackticksFencedCodeOpeningSegment::new("```\n", 0, 3, "")
        );
        success_case!(
            should_work_without_trailing_newline,
            "```",
            parsed => BackticksFencedCodeOpeningSegment::new("```", 0, 3, "")
        );
        success_case!(
            should_work_with_3_backticks_and_3_whitespace_ident,
            "   ```\n",
            parsed => BackticksFencedCodeOpeningSegment::new("   ```\n", 3, 3, "")
        );
        success_case!(
            should_work_with_info_string,
            "```rust\n",
            parsed => BackticksFencedCodeOpeningSegment::new("```rust\n", 0, 3, "rust",)
        );
        success_case!(
            should_work_with_info_string_without_trailing_newline,
            "```rust",
            parsed => BackticksFencedCodeOpeningSegment::new("```rust", 0, 3, "rust")
        );
        success_case!(
            should_work_with_padded_info_string,
            "```   rust is kind of fucking cool   \n",
            parsed => BackticksFencedCodeOpeningSegment::new(
                "```   rust is kind of fucking cool   \n",
                0,
                3,
                "rust is kind of fucking cool"
            )
        );
    }

    mod closing {
        use super::*;

        mod closes {
            use super::*;

            #[test]
            fn should_close_an_opening_fence_of_same_length() {
                let opening = BackticksFencedCodeOpeningSegment::new("```rust\n", 0, 3, "rust");
                let closing = BackticksFencedCodeClosingSegment::new("```\n", 0, 3);
                assert!(closing.closes(&opening));
            }

            #[test]
            fn should_close_an_opening_fence_of_smaller_length() {
                let opening = BackticksFencedCodeOpeningSegment::new("```rust\n", 0, 3, "rust");
                let closing = BackticksFencedCodeClosingSegment::new("````\n", 0, 4);
                assert!(closing.closes(&opening));
            }

            #[test]
            fn should_not_close_an_opening_fence_of_larger_length() {
                let opening = BackticksFencedCodeOpeningSegment::new("````\n", 0, 4, "rust");
                let closing = BackticksFencedCodeClosingSegment::new("```\n", 0, 3);
                assert!(!closing.closes(&opening));
            }
        }

        mod parse {
            use super::*;
            use crate::parse::test_utils::test_parse_macros;

            test_parse_macros!(BackticksFencedCodeClosingSegment);

            failure_case!(should_reject_empy, "");
            failure_case!(should_reject_blank_line, "\n");
            failure_case!(should_reject_2_backticks, "``\n");
            failure_case!(should_reject_info_string, "```rust\n");
            failure_case!(should_reject_4_whitespace_indent, "    ```\n");
            failure_case!(should_reject_tab_indent, "\t```\n");

            success_case!(
                should_work_with_3_backticks,
                "```\n",
                parsed => BackticksFencedCodeClosingSegment::new("```\n", 0, 3)
            );
            success_case!(
                should_work_without_trailing_newline,
                "```",
                parsed => BackticksFencedCodeClosingSegment::new("```", 0, 3)
            );
            success_case!(
                should_work_with_4_backticks,
                "````\n",
                parsed => BackticksFencedCodeClosingSegment::new("````\n", 0, 4)
            );
            success_case!(
                should_work_with_trailing_whitespaces,
                "```   \t\n",
                parsed => BackticksFencedCodeClosingSegment::new("```   \t\n", 0, 3)
            );
            success_case!(
                should_work_with_3_whitespaces_indent,
                "   ```\n",
                parsed => BackticksFencedCodeClosingSegment::new("   ```\n", 3, 3)
            );
        }
    }
}

mod utils {
    use parser::{ParseResult, Parser, is, rest, take_while, validate};

    pub fn backticks_fence(input: &str) -> ParseResult<&str, &str> {
        take_while(is('`')).at_least(3).parse(input)
    }

    pub fn info_string(input: &str) -> ParseResult<&str, &str> {
        let (remaining, parsed) = validate(rest, |s: &&str| {
            // The info string cannot contain backticks.
            !s.contains('`')
        })
        .parse(input)?;
        Ok((remaining, parsed.trim()))
    }

    #[cfg(test)]
    mod test {
        use super::*;

        mod backticks_fence {
            use super::*;

            macro_rules! failure_case {
                ($test:ident, $segment:expr) => {
                    #[test]
                    fn $test() {
                        assert!(backticks_fence($segment).is_err());
                    }
                };
            }

            macro_rules! success_case {
                ($test:ident, $segment:expr, $expected:expr) => {
                    #[test]
                    fn $test() {
                        assert_eq!(backticks_fence($segment), Ok(("", $expected)));
                    }
                };
            }

            failure_case!(should_reject_empty, "");
            failure_case!(should_reject_1_backtick, "`");
            failure_case!(should_reject_2_backticks, "``");

            success_case!(should_work_with_3_backticks, "```", "```");
            success_case!(should_work_with_4_backticks, "````", "````");
            success_case!(should_work_with_5_backticks, "`````", "`````");
        }
    }
}
