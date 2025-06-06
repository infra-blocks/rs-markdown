use crate::parse::blocks::open_block::IBlock;
use parser::{Map, Parser};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IndentedCode<'a> {
    opening: parse::IndentedCode<'a>,
    continuation: Option<Continuation<'a>>,
}

impl<'a> IndentedCode<'a> {
    fn new(opening: parse::IndentedCode<'a>, continuation: Option<Continuation<'a>>) -> Self {
        Self {
            opening,
            continuation,
        }
    }
}

impl<'a> IBlock<'a> for IndentedCode<'a> {
    type Open = open::IndentedCode<'a>;

    fn open(line: &'a str) -> parser::ParseResult<&'a str, Self::Open> {
        parse::indented_code
            .map(open::IndentedCode::new)
            .parse(line)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Continuation<'a> {
    continuation: Vec<parse::IndentedCodeOrBlankLine<'a>>,
    closing: parse::IndentedCode<'a>,
}

impl<'a> Continuation<'a> {
    fn new(
        continuation: Vec<parse::IndentedCodeOrBlankLine<'a>>,
        closing: parse::IndentedCode<'a>,
    ) -> Self {
        Self {
            continuation,
            closing,
        }
    }
}

pub mod open {
    use super::parse;
    use crate::parse::blocks::{
        block::BlankLine,
        open_block::{IOpenBlock, Staging},
    };
    use std::vec;

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct IndentedCode<'a> {
        opening: parse::IndentedCode<'a>,
        continuation: Vec<parse::IndentedCodeOrBlankLine<'a>>,
        staging: Staging<parse::IndentedCodeOrBlankLine<'a>>,
    }

    impl<'a> IndentedCode<'a> {
        pub(super) fn new(opening: parse::IndentedCode<'a>) -> Self {
            Self {
                opening,
                continuation: Vec::new(),
                staging: Staging::new(),
            }
        }
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct IndentedCodeCloseResult<'a> {
        pub indented_code: super::IndentedCode<'a>,
        // Trailing blank lines are removed from the indented code block and
        // returned separately.
        pub blank_lines: Vec<BlankLine<'a>>,
    }

    impl<'a> IndentedCodeCloseResult<'a> {
        fn new(indented_code: super::IndentedCode<'a>, blank_lines: Vec<BlankLine<'a>>) -> Self {
            Self {
                indented_code,
                blank_lines,
            }
        }
    }

    impl<'a>
        From<(
            parse::IndentedCode<'a>,
            Vec<parse::IndentedCodeOrBlankLine<'a>>,
        )> for IndentedCodeCloseResult<'a>
    {
        fn from(
            (indented_code, mut continuation): (
                parse::IndentedCode<'a>,
                Vec<parse::IndentedCodeOrBlankLine<'a>>,
            ),
        ) -> Self {
            let last_indented_code = continuation.iter().enumerate().rfind(|(_, segment)| {
                matches!(segment, parse::IndentedCodeOrBlankLine::IndentedCode(_))
            });
            match last_indented_code {
                Some((index, _)) => {
                    // If there are no trailing blank lines.
                    if index == continuation.len() - 1 {
                        let last_indented_code = continuation.pop().unwrap().unwrap_indented_code();
                        let continuation =
                            super::Continuation::new(continuation, last_indented_code);

                        return Self::new(
                            super::IndentedCode::new(indented_code, Some(continuation)),
                            vec![],
                        );
                    }
                    // Otherwise, we split the vector into two parts: up until the last indented code segment and the remaining blank lines.
                    let blank_lines = continuation.split_off(index + 1);
                    let blank_lines = blank_lines
                        .into_iter()
                        .map(|segment| segment.unwrap_blank_line())
                        .map(BlankLine::new)
                        .collect();
                    let last_indented_code = continuation.pop().unwrap().unwrap_indented_code();
                    let continuation = super::Continuation::new(continuation, last_indented_code);
                    Self::new(
                        super::IndentedCode::new(indented_code, Some(continuation)),
                        blank_lines,
                    )
                }
                // If we found no indented code, then all continuations are blank lines.
                None => Self::new(
                    super::IndentedCode::new(indented_code, None),
                    continuation
                        .into_iter()
                        .map(|segment| segment.unwrap_blank_line())
                        .map(BlankLine::new)
                        .collect(),
                ),
            }
        }
    }

    impl<'a> IOpenBlock<'a> for IndentedCode<'a> {
        type Closed = IndentedCodeCloseResult<'a>;

        fn stage(&mut self, line: &'a str) -> Result<&'a str, ()> {
            let (remaining, segment) = parse::indented_code_or_blank_line(line).map_err(|_| ())?;
            self.staging.set(segment);
            Ok(remaining)
        }

        fn commit(&mut self) {
            self.staging
                .commit(|segment| self.continuation.push(segment));
        }

        fn close<F: FnMut(Self::Closed)>(self, mut sink: F) {
            sink((self.opening, self.continuation).into());
        }
    }

    #[cfg(test)]
    mod test {
        use super::*;
        use crate::parse::blocks::{block::Continuation, open_block::IBlockTestExt};
        use parser::StrictParse;

        type Block<'a> = super::super::IndentedCode<'a>;
        type Result<'a> = <IndentedCode<'a> as IOpenBlock<'a>>::Closed;

        #[test]
        fn should_work_with_single_indented_code() {
            assert_eq!(
                Result::new(
                    Block::new(
                        parse::indented_code.strict_parse("    This is indented code.\n"),
                        None
                    ),
                    vec![]
                ),
                Block::open_and_close("    This is indented code.\n")
            );
        }

        #[test]
        fn should_work_with_closing_indented_code() {
            assert_eq!(
                Result::new(
                    Block::new(
                        parse::indented_code.strict_parse("    This is indented code.\n"),
                        Some(Continuation::new(
                            vec![parse::indented_code_or_blank_line.strict_parse("\n")],
                            parse::indented_code
                                .strict_parse("    There was a blank line in between.")
                        ))
                    ),
                    vec![]
                ),
                Block::open_and_close(
                    "    This is indented code.\n\n    There was a blank line in between.",
                )
            );
        }

        #[test]
        fn should_trim_off_trailing_blank_lines() {
            assert_eq!(
                Result::new(
                    Block::new(
                        parse::indented_code.strict_parse("    This is indented code.\n"),
                        Some(Continuation::new(
                            vec![
                                parse::indented_code_or_blank_line.strict_parse("\n"),
                                parse::indented_code_or_blank_line
                                    .strict_parse("    There was a blank line in between.\n"),
                                parse::indented_code_or_blank_line.strict_parse("\n"),
                            ],
                            parse::indented_code
                                .strict_parse("    Another one here. And more to follow.\n")
                        ))
                    ),
                    vec![
                        BlankLine::open_and_close("\n"),
                        BlankLine::open_and_close("\n")
                    ]
                ),
                Block::open_and_close(
                    "    This is indented code.\n\n    There was a blank line in between.\n\n    Another one here. And more to follow.\n\n\n",
                )
            );
        }
    }
}

mod parse {
    use crate::parse::{
        blocks::block::leaf::blank_line::parse::{BlankLine, blank_line},
        parsers::indented_by_at_least_4,
        predicates::is_blank_line,
    };
    use parser::{Map, Parser, one_of, recognize, rest};

    pub fn indented_code<'a>(input: &'a str) -> parser::ParseResult<&'a str, IndentedCode<'a>> {
        if is_blank_line(input) {
            return Err(input);
        }

        recognize((indented_by_at_least_4, rest))
            .map(IndentedCode::new)
            .parse(input)
    }

    /// An indented code segment.
    ///
    /// An indented code segment is one that starts with 4 spaces or a tab and
    /// isn't a blank line segment.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct IndentedCode<'a>(&'a str);

    impl<'a> IndentedCode<'a> {
        fn new(segment: &'a str) -> Self {
            Self(segment)
        }
    }

    pub fn indented_code_or_blank_line<'a>(
        input: &'a str,
    ) -> parser::ParseResult<&'a str, IndentedCodeOrBlankLine<'a>> {
        one_of((
            indented_code.map(IndentedCodeOrBlankLine::from),
            blank_line.map(IndentedCodeOrBlankLine::from),
        ))
        .parse(input)
    }

    /// An enum representing either an indented code segment or a blank line segment.
    ///
    /// This is useful in the context of building an indented code block, as it can
    /// contain blank lines.
    ///
    /// # Note
    /// Only non trailing blank lines should be kept in the block.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum IndentedCodeOrBlankLine<'a> {
        IndentedCode(IndentedCode<'a>),
        BlankLine(BlankLine<'a>),
    }

    impl<'a> IndentedCodeOrBlankLine<'a> {
        /// Unwraps the blank line segment, panicking if it is an indented code segment.
        pub fn unwrap_blank_line(self) -> BlankLine<'a> {
            match self {
                Self::IndentedCode(_) => panic!("cannot unwrap blank line on {:?}", self),
                Self::BlankLine(line) => line,
            }
        }

        /// Unwraps the indented code segment, panicking if it is a blank line segment.
        pub fn unwrap_indented_code(self) -> IndentedCode<'a> {
            match self {
                Self::IndentedCode(code) => code,
                Self::BlankLine(_) => panic!("cannot unwrap indented code on {:?}", self),
            }
        }
    }

    impl<'a> From<IndentedCode<'a>> for IndentedCodeOrBlankLine<'a> {
        fn from(indented_code: IndentedCode<'a>) -> Self {
            Self::IndentedCode(indented_code)
        }
    }

    impl<'a> From<BlankLine<'a>> for IndentedCodeOrBlankLine<'a> {
        fn from(blank_line: BlankLine<'a>) -> Self {
            Self::BlankLine(blank_line)
        }
    }

    #[cfg(test)]
    mod test {
        use super::*;

        mod indented_code {
            use super::*;
            use crate::parse::test_utils::test_parse_macros_2;

            test_parse_macros_2!(indented_code);

            failure_case!(should_reject_empty_segment, "");
            failure_case!(should_reject_blank_line, " \n");
            failure_case!(should_reject_3_whitespaces_indent, "   Missing one space\n");

            success_case!(
                should_work_with_4_whitespaces_indent,
                "    This is indented code. Finally.\n",
                IndentedCode::new("    This is indented code. Finally.\n")
            );
            success_case!(
                should_work_with_tab_indent,
                "\tThis is indented code. Finally.\n",
                IndentedCode::new("\tThis is indented code. Finally.\n")
            );
            success_case!(
                should_work_with_missing_eol,
                "    This is indented code. Finally.",
                IndentedCode::new("    This is indented code. Finally.")
            );
        }
        // TODO: test indented_code_or_blank_line
    }
}
