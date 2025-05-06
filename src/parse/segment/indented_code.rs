use super::blank_line::BlankLineSegment;
use crate::parse::{
    traits::{Parse, Segment},
    utils::{indented_by_at_least_4, line, non_whitespace},
};
use nom::{
    IResult, Parser,
    branch::alt,
    combinator::{recognize, rest},
    error::ParseError,
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
    BlankLine(BlankLineSegment<'a>),
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
    pub fn unwrap_blank_line(self) -> BlankLineSegment<'a> {
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

impl<'a> From<BlankLineSegment<'a>> for IndentedCodeOrBlankLineSegment<'a> {
    fn from(segment: BlankLineSegment<'a>) -> Self {
        Self::BlankLine(segment)
    }
}

impl<'a> Parse<'a> for IndentedCodeOrBlankLineSegment<'a> {
    fn parse<Error: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, Self, Error> {
        alt((
            IndentedCodeSegment::parse.map(Self::from),
            BlankLineSegment::parse.map(Self::from),
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
                        BlankLineSegment::parse_whole::<Error<&str>>(segment).unwrap()
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
