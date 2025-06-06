use crate::parse::blocks::open_block::IBlock;
use parser::{Map, Parser};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BlankLine<'a>(parse::BlankLine<'a>);

impl<'a> BlankLine<'a> {
    pub(super) fn new(segment: parse::BlankLine<'a>) -> Self {
        Self(segment)
    }
}

impl<'a> IBlock<'a> for BlankLine<'a> {
    type Open = open::BlankLine<'a>;

    fn open(line: &'a str) -> parser::ParseResult<&'a str, Self::Open> {
        parse::blank_line.map(open::BlankLine::new).parse(line)
    }
}

pub mod open {
    use super::parse;
    use crate::parse::blocks::open_block::SingleSegmentBlock;

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct BlankLine<'a>(parse::BlankLine<'a>);

    impl<'a> BlankLine<'a> {
        pub(super) fn new(segment: parse::BlankLine<'a>) -> Self {
            Self(segment)
        }
    }

    impl<'a> SingleSegmentBlock<'a> for BlankLine<'a> {
        type Closed = super::BlankLine<'a>;
    }

    impl<'a> From<BlankLine<'a>> for super::BlankLine<'a> {
        fn from(blank_line: BlankLine<'a>) -> Self {
            Self::new(blank_line.0)
        }
    }
}

pub mod parse {
    use crate::parse::parsers::{line_ending, space_or_tab};
    use parser::{Map, Parser, empty, one_of, recognize};

    pub fn blank_line<'a>(input: &'a str) -> parser::ParseResult<&'a str, BlankLine<'a>> {
        recognize(one_of((
            (space_or_tab(), line_ending),
            (space_or_tab().at_least(1), empty),
        )))
        .map(BlankLine::new)
        .parse(input)
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct BlankLine<'a>(&'a str);

    impl<'a> BlankLine<'a> {
        fn new(segment: &'a str) -> Self {
            Self(segment)
        }
    }

    #[cfg(test)]
    mod test {
        use super::*;
        use crate::parse::test_utils::test_parse_macros_2;

        test_parse_macros_2!(blank_line);

        failure_case!(should_reject_empty, "");
        failure_case!(should_reject_line_with_a_char, "    a\n");

        success_case!(should_work_with_one_whitespace, " ", BlankLine::new(" "));
        success_case!(
            should_work_with_a_single_newline,
            "\n",
            BlankLine::new("\n")
        );
        success_case!(should_work_with_a_single_tab, "\t", BlankLine::new("\t"));
        success_case!(
            should_work_with_any_whitespace,
            " \t\r\n",
            BlankLine::new(" \t\r\n")
        );
    }
}
