use crate::parse::blocks::open_block::IBlock;
use parser::{Map, ParseResult, Parser};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AtxHeading<'a> {
    /// The source segment from which this struct was constructed.
    segment: &'a str,
    /// The title of the heading, possibly empty.
    title: &'a str,
    /// The level of the heading, from 1 to 6.
    level: u8,
}

impl<'a> AtxHeading<'a> {
    fn new(segment: &'a str, title: &'a str, level: u8) -> Self {
        Self {
            segment,
            title,
            level,
        }
    }

    pub fn level(&self) -> u8 {
        self.level
    }

    pub fn title(&self) -> &'a str {
        self.title
    }
}

impl<'a> IBlock<'a> for AtxHeading<'a> {
    type Open = open::AtxHeading<'a>;

    fn open(line: &'a str) -> ParseResult<&'a str, Self::Open> {
        parse::atx_heading
            .map(|(segment, level, title): (&'a str, u8, &'a str)| {
                open::AtxHeading::new(segment, title, level)
            })
            .parse(line)
    }
}

pub mod open {
    use crate::parse::blocks::open_block::SingleSegmentBlock;

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct AtxHeading<'a> {
        /// The source segment from which this struct was constructed.
        segment: &'a str,
        /// The level of the heading, from 1 to 6.
        title: &'a str,
        /// The level of the heading, from 1 to 6.
        level: u8,
    }

    impl<'a> AtxHeading<'a> {
        pub(super) fn new(segment: &'a str, title: &'a str, level: u8) -> Self {
            Self {
                segment,
                title,
                level,
            }
        }
    }

    impl<'a> SingleSegmentBlock<'a> for AtxHeading<'a> {
        type Closed = super::AtxHeading<'a>;
    }

    impl<'a> From<AtxHeading<'a>> for super::AtxHeading<'a> {
        fn from(open: AtxHeading<'a>) -> Self {
            super::AtxHeading::new(open.segment, open.title, open.level)
        }
    }
}

mod parse {
    use crate::parse::parsers::{indented_by_less_than_4, line_ending_or_empty, space_or_tab};
    use parser::{Map, ParseResult, Parser, consumed, equals, one_of, recognize, rest, take_while};

    pub fn atx_heading(input: &str) -> ParseResult<&str, (&str, u8, &str)> {
        consumed((indented_by_less_than_4, hashes, title))
            .map(|(segment, (_, level, title))| (segment, level, title))
            .parse(input)
    }

    /// Parses the opening sequence and returns the amount of hashes found, which
    /// will be between 1 and 6.
    ///
    /// Note that if there are more than 6 hashes, this function does not fail.
    fn hashes(input: &str) -> ParseResult<&str, u8> {
        take_while(equals('#'))
            .at_least(1)
            .at_most(6)
            .map(|hashes: &str| {
                u8::try_from(hashes.len()).expect("unexpected hashes length greater than u8")
            })
            .parse(input)
    }

    /// Parses the title from the remaining input.
    ///
    /// It is assumed that the opening sequence has already been consumed and that the input
    /// starts immediately after it.
    ///
    /// A title will be invalid if it is not empty and does not start with a whitespace character.
    fn title<'a>(input: &'a str) -> ParseResult<&'a str, &'a str> {
        recognize(one_of((
            recognize((space_or_tab().at_least(1), rest)),
            line_ending_or_empty,
        )))
        .map(|title_segment: &'a str| extract_title(title_segment))
        .parse(input)
    }

    fn extract_title(segment: &str) -> &str {
        // Clean up the whitespaces around as they are not part of the title.
        let maybe_title = segment.trim();
        // Find the last word in the remaining possible title.
        let last_word_index = match maybe_title.rfind([' ', '\t']) {
            Some(last_space_index) => last_space_index + 1,
            None => 0,
        };
        // The last word is included unless it is a closing sequence.
        if is_closing_sequence(&maybe_title[last_word_index..]) {
            maybe_title[..last_word_index].trim()
        } else {
            maybe_title
        }
    }

    /// A closing sequence is a sequence of characters made entirely of hashes '#'.
    ///
    /// It requires at least one hash character.
    fn is_closing_sequence(input: &str) -> bool {
        if input.is_empty() {
            return false;
        }
        for char in input.chars() {
            if char != '#' {
                return false;
            }
        }
        true
    }

    #[cfg(test)]
    mod test {
        use super::*;

        use crate::parse::test_utils::test_parse_macros_2;

        test_parse_macros_2!(atx_heading);

        failure_case!(should_reject_empty_segment, "");
        failure_case!(should_reject_blank_line, "\n");
        failure_case!(should_reject_tab_indent, "\t# Heading\n");
        failure_case!(should_reject_4_whitespaces_prefix, "    # Heading\n");
        failure_case!(should_reject_missing_whitespace_before_title, "#hashtag\n");
        failure_case!(should_reject_if_not_just_hash_before_title, "#5 Heading\n");
        failure_case!(should_reject_7_hashes, "####### Heading\n");
        failure_case!(should_reject_escaped_hash, r"\## Heading\n");

        success_case!(
            should_work_with_simple_case,
            "# Heading\n",
            ("# Heading\n", 1, "Heading")
        );
        success_case!(
            should_work_with_2_hashes,
            "## Heading\n",
            ("## Heading\n", 2, "Heading")
        );
        success_case!(
            should_work_with_3_hashes,
            "### Heading\n",
            ("### Heading\n", 3, "Heading")
        );
        success_case!(
            should_work_with_4_hashes,
            "#### Heading\n",
            ("#### Heading\n", 4, "Heading")
        );
        success_case!(
            should_work_with_5_hashes,
            "##### Heading\n",
            ("##### Heading\n", 5, "Heading")
        );
        success_case!(
            should_work_with_6_hashes,
            "###### Heading\n",
            ("###### Heading\n", 6, "Heading")
        );
        success_case!(
            should_work_with_3_spaces_indent,
            "   # Heading\n",
            ("   # Heading\n", 1, "Heading")
        );
        success_case!(
            should_work_with_trailing_hashes,
            "# Heading ###  \t  \n",
            ("# Heading ###  \t  \n", 1, "Heading")
        );
        success_case!(
            should_include_trailing_hash_in_content_if_missing_whitespace,
            "# Heading#\n",
            ("# Heading#\n", 1, "Heading#")
        );
        success_case!(
            should_work_with_empty_heading_without_newline,
            "#",
            ("#", 1, "")
        );
        success_case!(
            should_work_with_empty_heading_with_newline,
            "#\n",
            ("#\n", 1, "")
        );
        success_case!(
            should_work_with_blank_heading,
            "#       \n",
            ("#       \n", 1, "")
        );
        success_case!(
            should_work_with_empty_heading_and_trailing_hashes,
            "## ###\n",
            ("## ###\n", 2, "")
        );
        success_case!(
            should_work_with_hash_content,
            "# ### #\n",
            ("# ### #\n", 1, "###")
        );
        success_case!(
            should_work_with_characters_after_what_appears_to_be_a_closing_sequence,
            "### foo ### b\n",
            ("### foo ### b\n", 3, "foo ### b")
        );
        success_case!(
            should_work_with_escaped_hash_as_content,
            "# Heading #\\##\n",
            ("# Heading #\\##\n", 1, "Heading #\\##")
        );
        success_case!(
            should_work_with_missing_eol,
            "# Heading",
            ("# Heading", 1, "Heading")
        );
    }
}
