use nom::{
    IResult, Parser,
    character::complete::space0,
    combinator::{consumed, eof},
    error::ParseError,
};

use crate::parse::{
    input::NomParse,
    traits::Segment,
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

impl<'a> NomParse<'a> for TildesFencedCodeOpeningSegment<'a> {
    fn nom_parse<Error: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, Self, Error> {
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

impl<'a> NomParse<'a> for TildesFencedCodeClosingSegment<'a> {
    fn nom_parse<Error: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, Self, Error> {
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
    use crate::parse::utils::is_char;
    use nom::{
        IResult, Parser, bytes::complete::take_while_m_n, combinator::rest, error::ParseError,
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
