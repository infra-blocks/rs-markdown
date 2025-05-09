// TODO: split into many modules.
use crate::{
    Segment,
    ast::BlankLine,
    parse::{
        traits::Parse,
        utils::{indented_by_at_least_4, line, non_whitespace},
    },
};
use nom::{
    IResult, Parser,
    branch::alt,
    combinator::{recognize, rest},
    error::ParseError,
    multi::{many0, many1},
};

/// An indented code segment.
///
/// An indented code segment is one that starts with 4 spaces or a tab and
/// isn't a blank line segment.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IndentedCodeSegment<'a>(&'a str);

impl<'a> IndentedCodeSegment<'a> {
    fn new(segment: &'a str) -> Self {
        Self(segment)
    }

    fn indented_code<Error: ParseError<&'a str>>(
        input: &'a str,
    ) -> IResult<&'a str, &'a str, Error> {
        recognize((indented_by_at_least_4, non_whitespace, rest)).parse(input)
    }
}

impl<'a> Parse<'a> for IndentedCodeSegment<'a> {
    fn parse<Error: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, Self, Error> {
        recognize(line.and_then(Self::indented_code))
            .map(Self::new)
            .parse(input)
    }
}

impl<'a> Segment<'a> for IndentedCodeSegment<'a> {
    fn segment(&self) -> &'a str {
        self.0
    }
}

/// An enum representing either an indented code segment or a blank line segment.
///
/// This is useful in the context of building an indented code block, as it can
/// contain blank lines.
///
/// # Note
/// Only non trailing blank lines should be kept in the block.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IndentedCodeOrBlankLineSegment<'a> {
    IndentedCode(IndentedCodeSegment<'a>),
    BlankLine(BlankLine<'a>),
}

impl<'a> IndentedCodeOrBlankLineSegment<'a> {
    #[allow(dead_code)]
    pub fn is_blank_line(&self) -> bool {
        matches!(self, Self::BlankLine(_))
    }

    #[allow(dead_code)]
    pub fn is_indented_code(&self) -> bool {
        matches!(self, Self::IndentedCode(_))
    }

    #[allow(dead_code)]
    pub fn unwrap_blank_line(self) -> BlankLine<'a> {
        if let Self::BlankLine(segment) = self {
            segment
        } else {
            panic!("cannot unwrap blank line from: {:?}", self)
        }
    }

    pub fn unwrap_indented_code(self) -> IndentedCodeSegment<'a> {
        if let Self::IndentedCode(segment) = self {
            segment
        } else {
            panic!("cannot unwrap indented code from: {:?}", self)
        }
    }
}

impl<'a> From<IndentedCodeSegment<'a>> for IndentedCodeOrBlankLineSegment<'a> {
    fn from(segment: IndentedCodeSegment<'a>) -> Self {
        Self::IndentedCode(segment)
    }
}

impl<'a> From<BlankLine<'a>> for IndentedCodeOrBlankLineSegment<'a> {
    fn from(segment: BlankLine<'a>) -> Self {
        Self::BlankLine(segment)
    }
}

impl<'a> Parse<'a> for IndentedCodeOrBlankLineSegment<'a> {
    fn parse<Error: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, Self, Error> {
        alt((
            IndentedCodeSegment::parse.map(Self::from),
            BlankLine::parse.map(Self::from),
        ))
        .parse(input)
    }
}

impl<'a> Segment<'a> for IndentedCodeOrBlankLineSegment<'a> {
    fn segment(&self) -> &'a str {
        match self {
            Self::IndentedCode(segment) => segment.segment(),
            Self::BlankLine(blank_line) => blank_line.segment(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContinuationSegments<'a> {
    pub segments: Vec<IndentedCodeOrBlankLineSegment<'a>>,
    pub closing_segment: IndentedCodeSegment<'a>,
}

impl<'a> ContinuationSegments<'a> {
    pub(crate) fn new(
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
            many1((many0(BlankLine::parse), IndentedCodeSegment::parse)).parse(input)?;

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

    mod indented_code_segment {
        use super::*;
        use nom::error::Error;

        macro_rules! failure_case {
            ($test:ident, $segment:expr) => {
                #[test]
                fn $test() {
                    assert!(IndentedCodeSegment::parse::<Error<&str>>($segment.clone()).is_err())
                }
            };
        }

        macro_rules! success_case {
            ($test:ident, $segment:expr) => {
                #[test]
                fn $test() {
                    assert_eq!(
                        IndentedCodeSegment::parse::<Error<&str>>($segment.clone()),
                        Ok(("", IndentedCodeSegment::new($segment)))
                    )
                }
            };
        }

        failure_case!(should_reject_empty_segment, "");
        failure_case!(should_reject_blank_line, " \n");
        failure_case!(should_reject_3_whitespaces_indent, "   Missing one space\n");

        success_case!(
            should_work_with_4_whitespaces_indent,
            "    This is indented code. Finally.\n"
        );
        success_case!(
            should_work_with_tab_indent,
            "\tThis is indented code. Finally.\n"
        );
        success_case!(
            should_work_with_missing_eol,
            "    This is indented code. Finally."
        );
    }

    // Test that it can accept an indented code or a blank line.
    mod indented_code_or_blank_line_segment {
        use crate::parse::traits::ParseWhole;

        use super::*;
        use nom::error::Error;

        #[test]
        fn should_reject_empty_segment() {
            assert!(IndentedCodeOrBlankLineSegment::parse::<Error<&str>>("").is_err())
        }

        #[test]
        fn should_work_with_single_char_blank_line() {
            let segment = " \n";
            assert_eq!(
                IndentedCodeOrBlankLineSegment::parse::<Error<&str>>(segment),
                Ok((
                    "",
                    IndentedCodeOrBlankLineSegment::BlankLine(
                        BlankLine::parse_whole::<Error<&str>>(segment).unwrap()
                    )
                ))
            )
        }

        #[test]
        fn should_work_with_indented_code() {
            let segment = "    This is indented code.\n";
            assert_eq!(
                IndentedCodeOrBlankLineSegment::parse::<Error<&str>>(segment),
                Ok((
                    "",
                    IndentedCodeOrBlankLineSegment::IndentedCode(
                        IndentedCodeSegment::parse_whole::<Error<&str>>(segment).unwrap()
                    )
                ))
            )
        }
    }
}
