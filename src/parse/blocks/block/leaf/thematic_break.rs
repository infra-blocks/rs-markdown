use crate::parse::blocks::open_block::IBlock;
use parser::{Map, Parser};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ThematicBreak<'a>(parse::ThematicBreak<'a>);

impl<'a> ThematicBreak<'a> {
    fn new(segment: parse::ThematicBreak<'a>) -> Self {
        Self(segment)
    }
}

impl<'a> IBlock<'a> for ThematicBreak<'a> {
    type Open = open::ThematicBreak<'a>;

    fn open(line: &'a str) -> parser::ParseResult<&'a str, Self::Open> {
        parse::thematic_break
            .map(open::ThematicBreak::new)
            .parse(line)
    }
}

pub mod open {
    use super::parse;
    use crate::parse::blocks::open_block::SingleSegmentBlock;

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct ThematicBreak<'a>(parse::ThematicBreak<'a>);

    impl<'a> ThematicBreak<'a> {
        pub(super) fn new(segment: parse::ThematicBreak<'a>) -> Self {
            Self(segment)
        }
    }

    impl<'a> SingleSegmentBlock<'a> for ThematicBreak<'a> {
        type Closed = super::ThematicBreak<'a>;
    }

    impl<'a> From<ThematicBreak<'a>> for super::ThematicBreak<'a> {
        fn from(blank_line: ThematicBreak<'a>) -> Self {
            Self::new(blank_line.0)
        }
    }
}

mod parse {
    use crate::parse::parsers::{indented_by_less_than_4, line_ending_or_empty, space_or_tab};
    use parser::{Map, ParseResult, Parser, is_one_of, one_of, recognize, tag, take_while};

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum ThematicBreak<'a> {
        Asterisks(&'a str),
        Hyphens(&'a str),
        Underscores(&'a str),
    }

    pub fn thematic_break(input: &str) -> ParseResult<&str, ThematicBreak<'_>> {
        one_of((asterisks, hyphens, underscores)).parse(input)
    }

    fn asterisks(input: &str) -> ParseResult<&str, ThematicBreak<'_>> {
        recognize((
            indented_by_less_than_4,
            tag("*"),
            space_or_tab(),
            tag("*"),
            space_or_tab(),
            tag("*"),
            take_while(is_one_of(&['*', ' ', '\t'])),
            line_ending_or_empty,
        ))
        .map(ThematicBreak::Asterisks)
        .parse(input)
    }

    fn hyphens(input: &str) -> ParseResult<&str, ThematicBreak<'_>> {
        recognize((
            indented_by_less_than_4,
            tag("-"),
            space_or_tab(),
            tag("-"),
            space_or_tab(),
            tag("-"),
            take_while(is_one_of(&['-', ' ', '\t'])),
            line_ending_or_empty,
        ))
        .map(ThematicBreak::Hyphens)
        .parse(input)
    }

    fn underscores(input: &str) -> ParseResult<&str, ThematicBreak<'_>> {
        recognize((
            indented_by_less_than_4,
            tag("_"),
            space_or_tab(),
            tag("_"),
            space_or_tab(),
            tag("_"),
            take_while(is_one_of(&['_', ' ', '\t'])),
            line_ending_or_empty,
        ))
        .map(ThematicBreak::Underscores)
        .parse(input)
    }

    #[cfg(test)]
    mod test {
        use super::*;
        use crate::parse::test_utils::test_parse_macros_2;

        test_parse_macros_2!(thematic_break);

        failure_case!(should_reject_empty, "");
        failure_case!(should_reject_blank_line, "  \n");
        failure_case!(should_reject_tab_indent, "\t---\n");
        failure_case!(should_reject_four_spaces_indent, "    ---\n");
        failure_case!(should_reject_non_consecutive_tokens, " -_*\n");
        failure_case!(should_reject_with_presence_of_other_characters, "---a\n");

        success_case!(
            should_work_with_three_underscores,
            "___\n",
            ThematicBreak::Underscores("___\n")
        );
        success_case!(
            should_work_with_four_underscores,
            "____\n",
            ThematicBreak::Underscores("____\n")
        );
        success_case!(
            should_work_with_three_hyphens,
            "---\n",
            ThematicBreak::Hyphens("---\n")
        );
        success_case!(
            should_work_with_four_hyphens,
            "----\n",
            ThematicBreak::Hyphens("----\n")
        );
        success_case!(
            should_work_with_three_asterisks,
            "***\n",
            ThematicBreak::Asterisks("***\n")
        );
        success_case!(
            should_work_with_four_asterisks,
            "****\n",
            ThematicBreak::Asterisks("****\n")
        );
        success_case!(
            should_work_with_three_spaces_indent,
            "   ---\n",
            ThematicBreak::Hyphens("   ---\n")
        );
        success_case!(
            should_work_with_trailing_whitespace,
            "--- \n",
            ThematicBreak::Hyphens("--- \n")
        );
        success_case!(
            should_work_with_spaces_interspersed,
            " - - -\n",
            ThematicBreak::Hyphens(" - - -\n")
        );
        success_case!(
            should_work_without_eol,
            "---",
            ThematicBreak::Hyphens("---")
        );
    }
}
