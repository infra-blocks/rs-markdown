use nom::{
    character::complete::space0,
    combinator::{consumed, eof},
    error::ParseError,
    IResult, Parser,
};

use crate::parse::{
    traits::{Parse, Segment},
    utils::{indented_by_less_than_4, line},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TildesFencedCodeOpeningSegment<'a> {
    segment: &'a str,
    indent: usize,
    // The amount of tildes used, minimally 3.
    fence_length: usize,
    info_string: &'a str,
}

impl<'a> TildesFencedCodeOpeningSegment<'a> {
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

impl<'a> Parse<'a> for TildesFencedCodeOpeningSegment<'a> {
    fn parse<Error: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, Self, Error> {
        consumed(line.and_then((
            indented_by_less_than_4,
            utils::tildes_fence,
            utils::info_string,
        )))
        .map(|(segment, (indent, fence, info_string))| {
            Self::new(segment, indent.len(), fence.len(), info_string)
        })
        .parse(input)
    }
}

impl<'a> Segment<'a> for TildesFencedCodeOpeningSegment<'a> {
    fn segment(&self) -> &'a str {
        self.segment
    }
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

impl<'a> Parse<'a> for TildesFencedCodeClosingSegment<'a> {
    fn parse<Error: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, Self, Error> {
        consumed(line.and_then((indented_by_less_than_4, utils::tildes_fence, space0, eof)))
            .map(|(segment, (indent, fence, _, _))| Self::new(segment, indent.len(), fence.len()))
            .parse(input)
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
        use nom::error::Error;

        macro_rules! failure_case {
            ($test:ident, $segment:expr) => {
                #[test]
                fn $test() {
                    assert!(
                        TildesFencedCodeOpeningSegment::parse::<Error<&str>>($segment).is_err(),
                    );
                }
            };
        }

        macro_rules! success_case {
            ($test:ident, $segment:expr, $expected:expr) => {
                #[test]
                fn $test() {
                    assert_eq!(
                        TildesFencedCodeOpeningSegment::parse::<Error<&str>>($segment),
                        Ok(("", $expected))
                    );
                }
            };
        }

        failure_case!(should_reject_empy, "");
        failure_case!(should_reject_blank_line, "\n");
        failure_case!(should_reject_2_tildes, "~~\n");
        failure_case!(should_reject_4_whitespace_indent, "    ~~~\n");
        failure_case!(should_reject_tab_indent, "\t~~~\n");

        success_case!(
            should_work_with_3_tildes,
            "~~~\n",
            TildesFencedCodeOpeningSegment::new("~~~\n", 0, 3, "")
        );
        success_case!(
            should_work_without_trailing_newline,
            "~~~",
            TildesFencedCodeOpeningSegment::new("~~~", 0, 3, "")
        );
        success_case!(
            should_work_with_3_tildes_and_3_whitespace_ident,
            "   ~~~\n",
            TildesFencedCodeOpeningSegment::new("   ~~~\n", 3, 3, "")
        );
        success_case!(
            should_work_with_info_string,
            "~~~rust\n",
            TildesFencedCodeOpeningSegment::new("~~~rust\n", 0, 3, "rust")
        );
        success_case!(
            should_work_with_info_string_without_trailing_newline,
            "~~~rust",
            TildesFencedCodeOpeningSegment::new("~~~rust", 0, 3, "rust")
        );
        success_case!(
            should_work_tildes_in_info_string,
            "~~~rust~\n",
            TildesFencedCodeOpeningSegment::new("~~~rust~\n", 0, 3, "rust~")
        );
        success_case!(
            should_work_backticks_in_info_string,
            "~~~rust`\n",
            TildesFencedCodeOpeningSegment::new("~~~rust`\n", 0, 3, "rust`")
        );
        success_case!(
            should_work_with_padded_info_string,
            "~~~   rust is kind of fucking cool   \n",
            TildesFencedCodeOpeningSegment::new(
                "~~~   rust is kind of fucking cool   \n",
                0,
                3,
                "rust is kind of fucking cool"
            )
        );
    }

    mod closing {
        use super::*;
        use nom::error::Error;

        macro_rules! failure_case {
            ($test:ident, $segment:expr) => {
                #[test]
                fn $test() {
                    assert!(
                        TildesFencedCodeClosingSegment::parse::<Error<&str>>($segment).is_err(),
                    );
                }
            };
        }

        macro_rules! success_case {
            ($test:ident, $segment:expr, $expected:expr) => {
                #[test]
                fn $test() {
                    assert_eq!(
                        TildesFencedCodeClosingSegment::parse::<Error<&str>>($segment),
                        Ok(("", $expected))
                    );
                }
            };
        }

        failure_case!(should_reject_empy, "");
        failure_case!(should_reject_blank_line, "\n");
        failure_case!(should_reject_2_tildes, "~~\n");
        failure_case!(should_reject_info_string, "~~~rust\n");
        failure_case!(should_reject_4_whitespace_indent, "    ~~~\n");
        failure_case!(should_reject_tab_indent, "\t~~~\n");

        success_case!(
            should_work_with_3_tildes,
            "~~~\n",
            TildesFencedCodeClosingSegment::new("~~~\n", 0, 3)
        );
        success_case!(
            should_work_without_trailing_newline,
            "~~~",
            TildesFencedCodeClosingSegment::new("~~~", 0, 3)
        );
        success_case!(
            should_work_with_4_tildes,
            "~~~~\n",
            TildesFencedCodeClosingSegment::new("~~~~\n", 0, 4)
        );
        success_case!(
            should_work_with_trailing_whitespaces,
            "~~~   \t\n",
            TildesFencedCodeClosingSegment::new("~~~   \t\n", 0, 3)
        );
        success_case!(
            should_work_with_3_whitespaces_indent,
            "   ~~~\n",
            TildesFencedCodeClosingSegment::new("   ~~~\n", 3, 3)
        );
    }
}

mod utils {
    use crate::parse::utils::is_char;
    use nom::{
        bytes::complete::take_while_m_n, combinator::rest, error::ParseError, IResult, Parser,
    };

    pub fn tildes_fence<'a, Error: ParseError<&'a str>>(
        input: &'a str,
    ) -> IResult<&'a str, &'a str, Error> {
        take_while_m_n(3, usize::MAX, is_char('~')).parse(input)
    }

    pub fn info_string<'a, Error: ParseError<&'a str>>(
        input: &'a str,
    ) -> IResult<&'a str, &'a str, Error> {
        let (remaining, info_string) = rest.parse(input)?;
        Ok((remaining, info_string.trim()))
    }
}
