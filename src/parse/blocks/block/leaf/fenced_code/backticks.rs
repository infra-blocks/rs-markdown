use crate::parse::blocks::open_block::IBlock;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BackticksFencedCode<'a> {
    opening: parse::Opening<'a>,
    content: Vec<&'a str>,
    /// The closing segment is allowed to be None in one scenario: when the end of input is reached
    /// before a closing segment. This is allowed by the spec.
    closing: Option<parse::Closing<'a>>,
}

impl<'a> BackticksFencedCode<'a> {
    fn new(
        opening: parse::Opening<'a>,
        content: Vec<&'a str>,
        closing: Option<parse::Closing<'a>>,
    ) -> Self {
        Self {
            opening,
            content,
            closing,
        }
    }
}

impl<'a> IBlock<'a> for BackticksFencedCode<'a> {
    type Open = open::BackticksFencedCode<'a>;

    fn open(line: &'a str) -> parser::ParseResult<&'a str, Self::Open> {
        let (remaining, opening) = parse::opening(line)?;
        Ok((remaining, Self::Open::new(opening)))
    }
}

pub mod open {
    use super::parse;
    use crate::parse::blocks::open_block::{IOpenBlock, Staging};
    use parser::Parser;

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct BackticksFencedCode<'a> {
        opening: parse::Opening<'a>,
        content: Vec<&'a str>,
        closing: Option<parse::Closing<'a>>,
        staging: Staging<parse::ContentOrClosing<'a>>,
    }

    impl<'a> BackticksFencedCode<'a> {
        pub(super) fn new(opening: parse::Opening<'a>) -> Self {
            Self {
                opening,
                content: Vec::new(),
                closing: None,
                staging: Staging::new(),
            }
        }
    }

    impl<'a> IOpenBlock<'a> for BackticksFencedCode<'a> {
        type Closed = super::BackticksFencedCode<'a>;

        fn stage(&mut self, line: &'a str) -> Result<&'a str, ()> {
            if self.closing.is_some() {
                // If we already have a closing segment, we cannot stage more content.
                return Err(());
            }

            let (remaining, content_or_closing) = parse::content_or_closing(&self.opening)
                .parse(line)
                .map_err(|_| ())?;
            self.staging.set(content_or_closing);
            Ok(remaining)
        }

        fn commit(&mut self) {
            self.staging
                .commit(|content_or_closing| match content_or_closing {
                    parse::ContentOrClosing::Content(content) => {
                        self.content.push(content);
                    }
                    parse::ContentOrClosing::Closing(closing) => {
                        self.closing = Some(closing);
                    }
                });
        }

        fn close<F: FnMut(Self::Closed)>(self, mut sink: F) {
            sink(super::BackticksFencedCode::new(
                self.opening,
                self.content,
                self.closing,
            ))
        }
    }

    #[cfg(test)]
    mod test {
        use super::*;
        use crate::parse::blocks::{
            block::leaf::fenced_code::backticks::parse::opening,
            open_block::{IBlockTestExt, IOpenBlockTestExt},
        };
        use parser::StrictParse;
        use std::vec;

        type Block<'a> = <BackticksFencedCode<'a> as IOpenBlock<'a>>::Closed;

        #[test]
        fn should_should_reject_staging_after_closing() {
            let mut block = Block::open_and_commit_all("```rust\n```");
            assert!(block.stage("let y = 43;\n").is_err());
        }

        #[test]
        fn should_work_when_closed_immediately() {
            assert_eq!(
                Block::new(opening.strict_parse("```rust\n"), vec![], None),
                Block::open_and_close("```rust\n")
            )
        }

        #[test]
        fn should_discard_staged_content_if_closed_before_committing() {
            let mut block = Block::strict_open("```rust\n");
            block.stage("let x = 42;\n").unwrap();
            assert_eq!(
                Block::new(opening.strict_parse("```rust\n"), vec![], None),
                block.close_and_return()
            )
        }

        #[test]
        fn should_work_when_closed_after_comitting_staged_content() {
            let block = Block::open_and_commit_all("```rust\nlet x = 42;\n");
            assert_eq!(
                Block::new(
                    opening.strict_parse("```rust\n"),
                    vec!["let x = 42;\n"],
                    None
                ),
                block.close_and_return()
            )
        }

        #[test]
        fn should_work_when_closed_with_comitted_closing() {
            let block = Block::open_and_commit_all("```rust\n```\n");
            assert_eq!(
                Block::new(
                    opening("```rust\n").map(|(_, segment)| segment).unwrap(),
                    vec![],
                    Some(parse::closing.strict_parse("```\n"))
                ),
                block.close_and_return()
            );
        }

        #[test]
        fn should_work_when_closed_with_content_and_closing() {
            let block = Block::open_and_commit_all("```rust\nlet x = 42;\n```\n");
            assert_eq!(
                Block::new(
                    opening.strict_parse("```rust\n"),
                    vec!["let x = 42;\n"],
                    Some(parse::closing.strict_parse("```\n"))
                ),
                block.close_and_return()
            );
        }
    }
}

mod parse {
    use crate::parse::parsers::{indented_by_less_than_4, line_ending_or_empty, space_or_tab};
    use parser::{Map, ParseResult, Parser, Validate, consumed};

    pub fn opening<'a>(input: &'a str) -> ParseResult<&'a str, Opening<'a>> {
        consumed((
            indented_by_less_than_4,
            utils::backticks_fence,
            utils::info_string,
        ))
        .map(
            |(segment, (indent, fence, info_string)): (&'a str, (&'a str, &'a str, &'a str))| {
                Opening::new(segment, indent.len(), fence.len(), info_string)
            },
        )
        .parse(input)
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct Opening<'a> {
        segment: &'a str,
        indent: usize,
        // The amount of backticks used, minimally 3.
        fence_length: usize,
        info_string: &'a str,
    }

    impl<'a> Opening<'a> {
        fn new(segment: &'a str, indent: usize, fence_length: usize, info_string: &'a str) -> Self {
            Self {
                segment,
                indent,
                fence_length,
                info_string,
            }
        }
    }

    pub fn closing<'a>(input: &'a str) -> ParseResult<&'a str, Closing<'a>> {
        consumed((
            indented_by_less_than_4,
            utils::backticks_fence,
            space_or_tab(),
            line_ending_or_empty,
        ))
        .map(
            |(segment, (indent, fence, _, _)): (&'a str, (&'a str, &'a str, &'a str, &'a str))| {
                Closing::new(segment, indent.len(), fence.len())
            },
        )
        .parse(input)
    }

    // Closing segments don't have info strings.
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct Closing<'a> {
        segment: &'a str,
        indent: usize,
        fence_length: usize,
    }

    impl<'a> Closing<'a> {
        /// Returns true if the closing segment is a valid closure for the opening segment.
        ///
        /// This is only true if the closing segment's fence is at least as long as the opening segment's fence.
        pub fn closes(&self, opening: &Opening) -> bool {
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

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub enum ContentOrClosing<'a> {
        Content(&'a str),
        Closing(Closing<'a>),
    }

    pub fn content_or_closing<'a>(
        opening: &Opening<'a>,
    ) -> impl Fn(&'a str) -> ParseResult<&'a str, ContentOrClosing<'a>> {
        |input: &str| {
            if input.is_empty() {
                return Err(input);
            }
            println!("received input: {input:?}");
            match closing
                .validate(|segment: &Closing| segment.closes(opening))
                .map(ContentOrClosing::Closing)
                .parse(input)
            {
                Ok((remaining, closing)) => Ok((remaining, closing)),
                Err(input) => Ok((input, ContentOrClosing::Content(input))),
            }
        }
    }

    #[cfg(test)]
    mod test {
        use super::*;

        mod opening {
            use super::*;
            use crate::parse::test_utils::test_parse_macros_2;

            test_parse_macros_2!(opening);

            failure_case!(should_reject_empy, "");
            failure_case!(should_reject_blank_line, "\n");
            failure_case!(should_reject_2_backticks, "``\n");
            failure_case!(should_reject_backticks_in_info_string, "```rust`\n");
            failure_case!(should_reject_4_whitespace_indent, "    ```\n");
            failure_case!(should_reject_tab_indent, "\t```\n");

            success_case!(
                should_work_with_3_backticks,
                "```\n",
                Opening::new("```\n", 0, 3, "")
            );
            success_case!(
                should_work_without_trailing_newline,
                "```",
                Opening::new("```", 0, 3, "")
            );
            success_case!(
                should_work_with_3_backticks_and_3_whitespace_ident,
                "   ```\n",
                Opening::new("   ```\n", 3, 3, "")
            );
            success_case!(
                should_work_with_info_string,
                "```rust\n",
                Opening::new("```rust\n", 0, 3, "rust",)
            );
            success_case!(
                should_work_with_info_string_without_trailing_newline,
                "```rust",
                Opening::new("```rust", 0, 3, "rust")
            );
            success_case!(
                should_work_with_padded_info_string,
                "```   rust is kind of fucking cool   \n",
                Opening::new(
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
                    let opening = Opening::new("```rust\n", 0, 3, "rust");
                    let closing = Closing::new("```\n", 0, 3);
                    assert!(closing.closes(&opening));
                }

                #[test]
                fn should_close_an_opening_fence_of_smaller_length() {
                    let opening = Opening::new("```rust\n", 0, 3, "rust");
                    let closing = Closing::new("````\n", 0, 4);
                    assert!(closing.closes(&opening));
                }

                #[test]
                fn should_not_close_an_opening_fence_of_larger_length() {
                    let opening = Opening::new("````\n", 0, 4, "rust");
                    let closing = Closing::new("```\n", 0, 3);
                    assert!(!closing.closes(&opening));
                }
            }

            mod parse {
                use super::*;
                use crate::parse::test_utils::test_parse_macros_2;

                test_parse_macros_2!(closing);

                failure_case!(should_reject_empy, "");
                failure_case!(should_reject_blank_line, "\n");
                failure_case!(should_reject_2_backticks, "``\n");
                failure_case!(should_reject_info_string, "```rust\n");
                failure_case!(should_reject_4_whitespace_indent, "    ```\n");
                failure_case!(should_reject_tab_indent, "\t```\n");

                success_case!(
                    should_work_with_3_backticks,
                    "```\n",
                    Closing::new("```\n", 0, 3)
                );
                success_case!(
                    should_work_without_trailing_newline,
                    "```",
                    Closing::new("```", 0, 3)
                );
                success_case!(
                    should_work_with_4_backticks,
                    "````\n",
                    Closing::new("````\n", 0, 4)
                );
                success_case!(
                    should_work_with_trailing_whitespaces,
                    "```   \t\n",
                    Closing::new("```   \t\n", 0, 3)
                );
                success_case!(
                    should_work_with_3_whitespaces_indent,
                    "   ```\n",
                    Closing::new("   ```\n", 3, 3)
                );
            }
        }
    }

    mod utils {
        use parser::{ParseResult, Parser, equals, rest, take_while, validate};

        pub fn backticks_fence(input: &str) -> ParseResult<&str, &str> {
            take_while(equals('`')).at_least(3).parse(input)
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
}
