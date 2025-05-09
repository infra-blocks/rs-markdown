use crate::{
    ast::AtxHeading,
    parse::{
        traits::Parse,
        utils::{indented_by_less_than_4, is_char, line},
    },
};
use nom::{
    IResult, Parser, bytes::complete::take_while_m_n, character::complete::space1,
    combinator::consumed, error::ParseError, sequence::preceded,
};

impl<'a> Parse<'a> for AtxHeading<'a> {
    fn parse<Error: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, Self, Error> {
        consumed(line.and_then(parts()))
            .map(|(text, (level, title))| Self::new(text, title, level))
            .parse(input)
    }
}

// TODO: use parse style syntax on the utility functions underneath and move to internal utils module.

/// Parses the parts of the ATX heading segment.
///
/// The parts are made of the opening sequence's length and the title.
/// The length will be between 1 and 6, inclusively and the title may
/// or may not be empty.
fn parts<'a, Error: ParseError<&'a str>>()
-> impl Parser<&'a str, Output = (u8, &'a str), Error = Error> {
    preceded(indented_by_less_than_4, (opening_sequence(), parse_title))
}

/// Parses the opening sequence and returns the amount of hashes found, which
/// will be between 1 and 6.
///
/// Note that if there are more than 6 hashes, this function does not fail.
fn opening_sequence<'a, Error: ParseError<&'a str>>()
-> impl Parser<&'a str, Output = u8, Error = Error> {
    take_while_m_n(1, 6, is_char('#')).map(|hashes: &str| {
        u8::try_from(hashes.len()).expect("unexpected hashes length greater than u8")
    })
}

/// Parses the title from the remaining input.
///
/// It is assumed that the opening sequence has already been consumed and that the input
/// starts immediately after it.
///
/// A title will be invalid if it is not empty and does not start with a whitespace character.
fn parse_title<'a, Error: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, &'a str, Error> {
    if input.is_empty() {
        return Ok(("", ""));
    }
    // The title sequence has to start with at least one space character, otherwise we don't have a valid
    // ATX heading.
    let (input, _) = space1(input)?;
    // Now that we know we start with a whitespace, we can remove the trailing ones.
    let input = input.trim_end();
    // The title is everything until the end of the line or a closing sequence.
    match input.rfind([' ', '\t']) {
        Some(last_space_index) => {
            // If the last word is a closing sequence, then the title is everything up until the last word, and trimmed again.
            if is_closing_sequence(&input[last_space_index + 1..]) {
                Ok(("", input[..last_space_index].trim()))
            } else {
                Ok(("", input))
            }
        }
        None => {
            if is_closing_sequence(input) {
                Ok(("", ""))
            } else {
                Ok(("", input))
            }
        }
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
        use crate::ast::AtxHeading;
        use nom::error::Error;

        macro_rules! failure_case {
            ($test:ident, $segment:expr) => {
                #[test]
                fn $test() {
                    assert!(AtxHeading::parse::<Error<&str>>($segment.clone()).is_err())
                }
            };
        }

        macro_rules! success_case {
            ($test:ident, $segment:expr, $expected:expr) => {
                #[test]
                fn $test() {
                    assert_eq!(
                        AtxHeading::parse::<Error<&str>>($segment.clone()),
                        Ok(("", $expected))
                    )
                }
            };
        }

        failure_case!(should_reject_empty_segment, "");
        failure_case!(should_reject_blank_line, "\n");
        failure_case!(should_reject_tab_indent, "\t# Heading\n");
        failure_case!(should_reject_4_whitespaces_prefix, "    # Heading\n");
        failure_case!(
            should_reject_missing_whitespace_before_content,
            "#hashtag\n"
        );
        failure_case!(
            should_reject_if_not_just_hash_before_content,
            "#5 Heading\n"
        );
        failure_case!(should_reject_7_hashes, "####### Heading\n");
        failure_case!(should_reject_escaped_hash, r"\## Heading\n");

        success_case!(
            should_work_with_simple_case,
            "# Heading\n",
            AtxHeading::new("# Heading\n", "Heading", 1)
        );
        success_case!(
            should_work_with_2_hashes,
            "## Heading\n",
            AtxHeading::new("## Heading\n", "Heading", 2)
        );
        success_case!(
            should_work_with_3_hashes,
            "### Heading\n",
            AtxHeading::new("### Heading\n", "Heading", 3)
        );
        success_case!(
            should_work_with_4_hashes,
            "#### Heading\n",
            AtxHeading::new("#### Heading\n", "Heading", 4)
        );
        success_case!(
            should_work_with_5_hashes,
            "##### Heading\n",
            AtxHeading::new("##### Heading\n", "Heading", 5)
        );
        success_case!(
            should_work_with_6_hashes,
            "###### Heading\n",
            AtxHeading::new("###### Heading\n", "Heading", 6)
        );
        success_case!(
            should_work_with_3_spaces_indent,
            "   # Heading\n",
            AtxHeading::new("   # Heading\n", "Heading", 1)
        );
        success_case!(
            should_work_with_trailing_hashes,
            "# Heading ###  \t  \n",
            AtxHeading::new("# Heading ###  \t  \n", "Heading", 1)
        );
        success_case!(
            should_include_trailing_hash_in_content_if_missing_whitespace,
            "# Heading#\n",
            AtxHeading::new("# Heading#\n", "Heading#", 1)
        );
        success_case!(
            should_work_with_empty_heading_without_newline,
            "#",
            AtxHeading::new("#", "", 1)
        );
        success_case!(
            should_work_with_blank_heading,
            "#       \n",
            AtxHeading::new("#       \n", "", 1)
        );
        success_case!(
            should_work_with_empty_heading_and_trailing_hashes,
            "## ###\n",
            AtxHeading::new("## ###\n", "", 2)
        );
        success_case!(
            should_work_with_hash_content,
            "# ### #\n",
            AtxHeading::new("# ### #\n", "###", 1)
        );
        success_case!(
            should_work_with_characters_after_what_appears_to_be_a_closing_sequence,
            "### foo ### b\n",
            AtxHeading::new("### foo ### b\n", "foo ### b", 3)
        );
        success_case!(
            should_work_with_escaped_hash_as_content,
            "# Heading #\\##\n",
            AtxHeading::new("# Heading #\\##\n", "Heading #\\##", 1)
        );
        success_case!(
            should_work_with_missing_eol,
            "# Heading",
            AtxHeading::new("# Heading", "Heading", 1)
        );
    }
}
