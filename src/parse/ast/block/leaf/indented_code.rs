use std::iter::{self};

use nom::{
    error::ParseError,
    multi::{many0, many1},
    IResult, Parser,
};

use crate::parse::{
    segment::{
        blank_line::BlankLineSegment,
        indented_code::{IndentedCodeOrBlankLineSegment, IndentedCodeSegment},
    },
    traits::{Parse, Segments},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IndentedCode<'a> {
    opening_segment: IndentedCodeSegment<'a>,
    continuation_segments: Option<ContinuationSegments<'a>>,
}

impl<'a> IndentedCode<'a> {
    fn new(
        opening_segment: IndentedCodeSegment<'a>,
        continuation_segments: Option<ContinuationSegments<'a>>,
    ) -> Self {
        Self {
            opening_segment,
            continuation_segments,
        }
    }

    fn single_segment(opening_segment: IndentedCodeSegment<'a>) -> Self {
        Self::new(opening_segment, None)
    }

    fn multi_segments(
        opening_segment: IndentedCodeSegment<'a>,
        continuation_segments: ContinuationSegments<'a>,
    ) -> Self {
        Self::new(opening_segment, Some(continuation_segments))
    }
}

impl<'a> Parse<'a> for IndentedCode<'a> {
    fn parse<Error: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, Self, Error> {
        let (remaining, opening_segment) = IndentedCodeSegment::parse::<Error>(input)?;
        match ContinuationSegments::parse::<Error>(remaining) {
            Ok((remaining, continuation_segments)) => {
                let indented_code =
                    IndentedCode::multi_segments(opening_segment, continuation_segments);
                Ok((remaining, indented_code))
            }
            Err(_) => {
                // If there are no continuation segments, we just return the opening segment.
                let indented_code = IndentedCode::single_segment(opening_segment);
                Ok((remaining, indented_code))
            }
        }
    }
}

impl<'a> Segments<'a> for IndentedCode<'a> {
    type SegmentsIter = IndentedCodeTextIterator<'a>;

    fn segments(&'a self) -> Self::SegmentsIter {
        IndentedCodeTextIterator::from(self)
    }
}

pub struct IndentedCodeTextIterator<'a> {
    opening_segment: Option<&'a str>,
    continuation_segments: Box<dyn Iterator<Item = &'a str> + 'a>,
    closing_segment: Option<&'a str>,
}

impl<'a> IndentedCodeTextIterator<'a> {
    fn new(
        opening_segment: &'a str,
        continuation_segments: Box<dyn Iterator<Item = &'a str> + 'a>,
        closing_segment: Option<&'a str>,
    ) -> Self {
        Self {
            opening_segment: Some(opening_segment),
            continuation_segments,
            closing_segment,
        }
    }
}

impl<'a> From<&'a IndentedCode<'a>> for IndentedCodeTextIterator<'a> {
    fn from(indented_code: &'a IndentedCode<'a>) -> Self {
        let opening_segment = indented_code.opening_segment.0;
        match &indented_code.continuation_segments {
            None => Self::new(opening_segment, Box::new(iter::empty()), None),
            Some(continuation_segments) => {
                let closing_segment = continuation_segments.closing_segment.0;
                let continuation_segments = continuation_segments
                    .segments
                    .iter()
                    .map(|segment| segment.text());
                Self::new(
                    opening_segment,
                    Box::new(continuation_segments),
                    Some(closing_segment),
                )
            }
        }
    }
}

impl<'a> Iterator for IndentedCodeTextIterator<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(text) = self.opening_segment.take() {
            return Some(text);
        }
        if let Some(text) = self.continuation_segments.next() {
            return Some(text);
        }
        self.closing_segment.take()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContinuationSegments<'a> {
    pub segments: Vec<IndentedCodeOrBlankLineSegment<'a>>,
    pub closing_segment: IndentedCodeSegment<'a>,
}

impl<'a> ContinuationSegments<'a> {
    fn new(
        segments: Vec<IndentedCodeOrBlankLineSegment<'a>>,
        closing_segment: IndentedCodeSegment<'a>,
    ) -> Self {
        Self {
            segments,
            closing_segment,
        }
    }
}

impl<'a> Parse<'a> for ContinuationSegments<'a> {
    fn parse<Error: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, Self, Error> {
        let (remaining, blocks) =
            many1((many0(BlankLineSegment::parse), IndentedCodeSegment::parse)).parse(input)?;

        let mut segments = Vec::new();
        for (blank_lines, indented_code_segment) in blocks {
            segments.extend(
                blank_lines
                    .into_iter()
                    .map(IndentedCodeOrBlankLineSegment::from),
            );
            segments.push(IndentedCodeOrBlankLineSegment::from(indented_code_segment));
        }
        // The last segment is guaranteed to be an indented code segment given our algorithm.
        let closing_segment = segments.pop().unwrap().unwrap_indented_code();
        let continuation_segments = ContinuationSegments::new(segments, closing_segment);
        Ok((remaining, continuation_segments))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    // Tests that it properly strips off trailing blank lines when present.
    mod parse {
        use super::*;
        use crate::parse::traits::ParseWhole;
        use nom::error::Error;

        macro_rules! failure_case {
            ($test:ident, $segment:expr) => {
                #[test]
                fn $test() {
                    assert!(IndentedCode::parse::<Error<&str>>($segment.clone()).is_err())
                }
            };
        }

        macro_rules! success_case {
            ($test:ident, $segment:expr, $expected:expr) => {
                success_case!($test, $segment, $expected, "")
            };
            ($test:ident, $segment:expr, $expected:expr, $remaining:expr) => {
                #[test]
                fn $test() {
                    let (remaining, indented_code) =
                        IndentedCode::parse::<Error<&str>>($segment.clone()).unwrap();
                    assert_eq!(indented_code, $expected);
                    assert_eq!(remaining, $remaining);
                }
            };
        }

        failure_case!(should_fail_with_empty_string, "");
        failure_case!(should_fail_with_blank_line, " \n");
        failure_case!(
            should_fail_with_3_spaces,
            "   This is not exactly indented code.\n"
        );

        success_case!(
            should_work_with_one_indented_code_segment,
            "    This is indented code\nThis is not.",
            IndentedCode::single_segment(
                IndentedCodeSegment::parse_whole::<Error<&str>>("    This is indented code\n")
                    .unwrap()
            ),
            "This is not."
        );
        success_case!(
            should_work_with_tab_indent,
            "\tThis is indented code\nThis is not.",
            IndentedCode::single_segment(
                IndentedCodeSegment::parse_whole::<Error<&str>>("\tThis is indented code\n")
                    .unwrap()
            ),
            "This is not."
        );
        success_case!(
            should_work_with_several_indentations,
            "  \t  \tThis is indented code\nThis is not.",
            IndentedCode::single_segment(
                IndentedCodeSegment::parse_whole::<Error<&str>>("  \t  \tThis is indented code\n")
                    .unwrap()
            ),
            "This is not."
        );
        success_case!(
            should_strip_off_trailing_blank_line,
            "    This is indented code\n \n",
            IndentedCode::single_segment(
                IndentedCodeSegment::parse_whole::<Error<&str>>("    This is indented code\n")
                    .unwrap()
            ),
            " \n"
        );
        success_case!(
            should_work_with_blank_lines_interleaved,
            r"    This is indented code.
 
    That blank line is part of the block too.
 
    And so was that one.
 
But not this one.",
            IndentedCode::multi_segments(
                IndentedCodeSegment::parse_whole::<Error<&str>>("    This is indented code.\n")
                    .unwrap(),
                ContinuationSegments::new(
                    vec![
                        IndentedCodeOrBlankLineSegment::from(
                            BlankLineSegment::parse_whole::<Error<&str>>(" \n").unwrap()
                        ),
                        IndentedCodeOrBlankLineSegment::from(
                            IndentedCodeSegment::parse_whole::<Error<&str>>(
                                "    That blank line is part of the block too.\n"
                            )
                            .unwrap()
                        ),
                        IndentedCodeOrBlankLineSegment::from(
                            BlankLineSegment::parse_whole::<Error<&str>>(" \n").unwrap()
                        )
                    ],
                    IndentedCodeSegment::parse_whole::<Error<&str>>("    And so was that one.\n")
                        .unwrap(),
                )
            ),
            " \nBut not this one."
        );
    }

    mod text {
        use super::*;
        use crate::parse::traits::ParseWhole;
        use nom::error::Error;
        use std::vec;

        #[test]
        fn should_work_with_single_segment() {
            let indented_code =
                IndentedCode::parse_whole::<Error<&str>>("    This is indented code\n").unwrap();
            let segments = indented_code.segments().collect::<Vec<_>>();
            assert_eq!(segments, vec!["    This is indented code\n"]);
        }

        #[test]
        fn should_work_with_multiple_segments() {
            let indented_code = IndentedCode::parse_whole::<Error<&str>>(
                r"    This is indented code

    This is the closing segment.",
            )
            .unwrap();
            let segments = indented_code.segments().collect::<Vec<_>>();
            assert_eq!(
                segments,
                vec![
                    "    This is indented code\n",
                    "\n",
                    "    This is the closing segment."
                ]
            );
        }
    }
}
