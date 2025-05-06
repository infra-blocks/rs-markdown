use nom::character::complete::space0;
use nom::combinator::eof;
use nom::Parser;
use nom::{combinator::consumed, error::ParseError, IResult};

use crate::parse::traits::{Parse, Segment};
use crate::parse::utils::{indented_by_less_than_4, line};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BackticksFencedCodeOpeningSegment<'a> {
    segment: &'a str,
    indent: usize,
    // The amount of backticks used, minimally 3.
    fence_length: usize,
    info_string: &'a str,
}

impl<'a> BackticksFencedCodeOpeningSegment<'a> {
    pub fn indent(&self) -> usize {
        self.indent
    }

    pub fn fence_length(&self) -> usize {
        self.fence_length
    }

    pub fn info_string(&self) -> &'a str {
        self.info_string
    }

    fn new(segment: &'a str, indent: usize, fence_length: usize, info_string: &'a str) -> Self {
        Self {
            segment,
            indent,
            fence_length,
            info_string,
        }
    }
}

impl<'a> Parse<'a> for BackticksFencedCodeOpeningSegment<'a> {
    fn parse<Error: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, Self, Error> {
        consumed(line.and_then((
            indented_by_less_than_4,
            utils::backticks_fence,
            utils::info_string,
        )))
        .map(|(segment, (indent, fence, info_string))| {
            Self::new(segment, indent.len(), fence.len(), info_string)
        })
        .parse(input)
    }
}

impl<'a> Segment<'a> for BackticksFencedCodeOpeningSegment<'a> {
    fn segment(&self) -> &'a str {
        self.segment
    }
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

    pub fn indent(&self) -> usize {
        self.indent
    }

    pub fn fence_length(&self) -> usize {
        self.fence_length
    }

    fn new(segment: &'a str, indent: usize, fence_length: usize) -> Self {
        Self {
            segment,
            indent,
            fence_length,
        }
    }
}

impl<'a> Parse<'a> for BackticksFencedCodeClosingSegment<'a> {
    fn parse<Error: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, Self, Error> {
        consumed(line.and_then((indented_by_less_than_4, utils::backticks_fence, space0, eof)))
            .map(|(segment, (indent, fence, _, _))| Self::new(segment, indent.len(), fence.len()))
            .parse(input)
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
        use nom::error::Error;

        macro_rules! failure_case {
            ($test:ident, $segment:expr) => {
                #[test]
                fn $test() {
                    assert!(
                        BackticksFencedCodeOpeningSegment::parse::<Error<&str>>($segment).is_err(),
                    );
                }
            };
        }

        macro_rules! success_case {
            ($test:ident, $segment:expr, $expected:expr) => {
                #[test]
                fn $test() {
                    assert_eq!(
                        BackticksFencedCodeOpeningSegment::parse::<Error<&str>>($segment),
                        Ok(("", $expected))
                    );
                }
            };
        }

        failure_case!(should_reject_empy, "");
        failure_case!(should_reject_blank_line, "\n");
        failure_case!(should_reject_2_backticks, "``\n");
        failure_case!(should_reject_backticks_in_info_string, "```rust`\n");
        failure_case!(should_reject_4_whitespace_indent, "    ```\n");
        failure_case!(should_reject_tab_indent, "\t```\n");

        success_case!(
            should_work_with_3_backticks,
            "```\n",
            BackticksFencedCodeOpeningSegment::new("```\n", 0, 3, "")
        );
        success_case!(
            should_work_without_trailing_newline,
            "```",
            BackticksFencedCodeOpeningSegment::new("```", 0, 3, "")
        );
        success_case!(
            should_work_with_3_backticks_and_3_whitespace_ident,
            "   ```\n",
            BackticksFencedCodeOpeningSegment::new("   ```\n", 3, 3, "")
        );
        success_case!(
            should_work_with_info_string,
            "```rust\n",
            BackticksFencedCodeOpeningSegment::new("```rust\n", 0, 3, "rust",)
        );
        success_case!(
            should_work_with_info_string_without_trailing_newline,
            "```rust",
            BackticksFencedCodeOpeningSegment::new("```rust", 0, 3, "rust")
        );
        success_case!(
            should_work_with_padded_info_string,
            "```   rust is kind of fucking cool   \n",
            BackticksFencedCodeOpeningSegment::new(
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
            use nom::error::Error;

            macro_rules! failure_case {
                ($test:ident, $segment:expr) => {
                    #[test]
                    fn $test() {
                        assert!(
                            BackticksFencedCodeClosingSegment::parse::<Error<&str>>($segment)
                                .is_err(),
                        );
                    }
                };
            }

            macro_rules! success_case {
                ($test:ident, $segment:expr, $expected:expr) => {
                    #[test]
                    fn $test() {
                        assert_eq!(
                            BackticksFencedCodeClosingSegment::parse::<Error<&str>>($segment),
                            Ok(("", $expected))
                        );
                    }
                };
            }

            failure_case!(should_reject_empy, "");
            failure_case!(should_reject_blank_line, "\n");
            failure_case!(should_reject_2_backticks, "``\n");
            failure_case!(should_reject_info_string, "```rust\n");
            failure_case!(should_reject_4_whitespace_indent, "    ```\n");
            failure_case!(should_reject_tab_indent, "\t```\n");

            success_case!(
                should_work_with_3_backticks,
                "```\n",
                BackticksFencedCodeClosingSegment::new("```\n", 0, 3)
            );
            success_case!(
                should_work_without_trailing_newline,
                "```",
                BackticksFencedCodeClosingSegment::new("```", 0, 3)
            );
            success_case!(
                should_work_with_4_backticks,
                "````\n",
                BackticksFencedCodeClosingSegment::new("````\n", 0, 4)
            );
            success_case!(
                should_work_with_trailing_whitespaces,
                "```   \t\n",
                BackticksFencedCodeClosingSegment::new("```   \t\n", 0, 3)
            );
            success_case!(
                should_work_with_3_whitespaces_indent,
                "   ```\n",
                BackticksFencedCodeClosingSegment::new("   ```\n", 3, 3)
            );
        }
    }
}

mod utils {
    use crate::parse::utils::is_char;
    use nom::{
        bytes::complete::take_while_m_n,
        combinator::{rest, verify},
        error::ParseError,
        IResult, Parser,
    };

    pub fn backticks_fence<'a, Error: ParseError<&'a str>>(
        input: &'a str,
    ) -> IResult<&'a str, &'a str, Error> {
        take_while_m_n(3, usize::MAX, is_char('`')).parse(input)
    }

    pub fn info_string<'a, Error: ParseError<&'a str>>(
        input: &'a str,
    ) -> IResult<&'a str, &'a str, Error> {
        let (remaining, parsed) = verify(rest, |s: &str| {
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
            use nom::error::Error;

            macro_rules! failure_case {
                ($test:ident, $segment:expr) => {
                    #[test]
                    fn $test() {
                        assert!(backticks_fence::<Error<&str>>($segment).is_err());
                    }
                };
            }

            macro_rules! success_case {
                ($test:ident, $segment:expr, $expected:expr) => {
                    #[test]
                    fn $test() {
                        assert_eq!(
                            backticks_fence::<Error<&str>>($segment),
                            Ok(("", $expected))
                        );
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
