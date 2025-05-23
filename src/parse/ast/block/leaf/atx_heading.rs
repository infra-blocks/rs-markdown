use crate::{
    ast::block::AtxHeading,
    parse::{
        parser_utils::{indented_by_less_than_4, line_ending_or_eof, space_or_tab},
        traits::ParseLine,
    },
};
use parser::{Map, ParseResult, Parser, consumed, is, one_of, recognize, rest, take_while};

pub fn atx_heading(input: &str) -> ParseResult<&str, AtxHeading> {
    consumed((indented_by_less_than_4, hashes, title))
        .map(|(segment, (_, level, title))| AtxHeading::new(segment, title, level))
        .parse(input)
}

impl<'a> ParseLine<'a> for AtxHeading<'a> {
    fn parse_line(input: &'a str) -> ParseResult<&'a str, Self> {
        atx_heading(input)
    }
}

/// Parses the opening sequence and returns the amount of hashes found, which
/// will be between 1 and 6.
///
/// Note that if there are more than 6 hashes, this function does not fail.
fn hashes(input: &str) -> ParseResult<&str, u8> {
    take_while(is('#'))
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
        line_ending_or_eof,
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

    mod parse {
        use super::*;
        use crate::parse::test_utils::test_parse_macros;

        test_parse_macros!(AtxHeading);

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
            parsed => AtxHeading::new("# Heading\n", "Heading", 1)
        );
        success_case!(
            should_work_with_2_hashes,
            "## Heading\n",
            parsed => AtxHeading::new("## Heading\n", "Heading", 2)
        );
        success_case!(
            should_work_with_3_hashes,
            "### Heading\n",
            parsed => AtxHeading::new("### Heading\n", "Heading", 3)
        );
        success_case!(
            should_work_with_4_hashes,
            "#### Heading\n",
            parsed => AtxHeading::new("#### Heading\n", "Heading", 4)
        );
        success_case!(
            should_work_with_5_hashes,
            "##### Heading\n",
            parsed => AtxHeading::new("##### Heading\n", "Heading", 5)
        );
        success_case!(
            should_work_with_6_hashes,
            "###### Heading\n",
            parsed => AtxHeading::new("###### Heading\n", "Heading", 6)
        );
        success_case!(
            should_work_with_3_spaces_indent,
            "   # Heading\n",
            parsed => AtxHeading::new("   # Heading\n", "Heading", 1)
        );
        success_case!(
            should_work_with_trailing_hashes,
            "# Heading ###  \t  \n",
            parsed => AtxHeading::new("# Heading ###  \t  \n", "Heading", 1)
        );
        success_case!(
            should_include_trailing_hash_in_content_if_missing_whitespace,
            "# Heading#\n",
            parsed => AtxHeading::new("# Heading#\n", "Heading#", 1)
        );
        success_case!(
            should_work_with_empty_heading_without_newline,
            "#",
            parsed => AtxHeading::new("#", "", 1)
        );
        success_case!(
            should_work_with_empty_heading_with_newline,
            "#\n",
            parsed => AtxHeading::new("#\n", "", 1)
        );
        success_case!(
            should_work_with_blank_heading,
            "#       \n",
            parsed => AtxHeading::new("#       \n", "", 1)
        );
        success_case!(
            should_work_with_empty_heading_and_trailing_hashes,
            "## ###\n",
            parsed => AtxHeading::new("## ###\n", "", 2)
        );
        success_case!(
            should_work_with_hash_content,
            "# ### #\n",
            parsed => AtxHeading::new("# ### #\n", "###", 1)
        );
        success_case!(
            should_work_with_characters_after_what_appears_to_be_a_closing_sequence,
            "### foo ### b\n",
            parsed => AtxHeading::new("### foo ### b\n", "foo ### b", 3)
        );
        success_case!(
            should_work_with_escaped_hash_as_content,
            "# Heading #\\##\n",
            parsed => AtxHeading::new("# Heading #\\##\n", "Heading #\\##", 1)
        );
        success_case!(
            should_work_with_missing_eol,
            "# Heading",
            parsed => AtxHeading::new("# Heading", "Heading", 1)
        );
    }
}
