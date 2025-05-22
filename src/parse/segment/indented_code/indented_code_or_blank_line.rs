use super::IndentedCodeSegment;
use crate::{
    Segment,
    ast::block::BlankLine,
    parse::{
        parser::{Map, ParseResult, Parser, one_of},
        traits::ParseLine,
    },
};

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
            panic!("cannot unwrap blank line from: {self:?}")
        }
    }

    pub fn unwrap_indented_code(self) -> IndentedCodeSegment<'a> {
        if let Self::IndentedCode(segment) = self {
            segment
        } else {
            panic!("cannot unwrap indented code from: {self:?}")
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

impl<'a> ParseLine<'a> for IndentedCodeOrBlankLineSegment<'a> {
    fn parse_line(input: &'a str) -> ParseResult<&'a str, Self> {
        one_of((
            IndentedCodeSegment::parse_line.map(Self::from),
            BlankLine::parse_line.map(Self::from),
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
    use crate::parse::test_utils::{StrictParse, test_parse_macros};

    test_parse_macros!(IndentedCodeOrBlankLineSegment);

    failure_case!(should_reject_empty, "");

    success_case!(
        should_work_with_blank_line,
        " \n",
        parsed => IndentedCodeOrBlankLineSegment::BlankLine(BlankLine::strict_parse(" \n")),
        ""
    );
    success_case!(
        should_work_with_indented_code,
        "    This is indented code.\n",
        parsed => IndentedCodeOrBlankLineSegment::IndentedCode(
            IndentedCodeSegment::strict_parse("    This is indented code.\n")
        ),
        ""
    );
}
