use super::predicates::is_space_or_tab;
use parser::{
    IsEmpty, ItemsIndices, ParseResult, Parser, PrefixEnd, SplitAt, SubsetRange, TakeWhileParser,
    empty, one_of, recognize, tag, take, take_while, validate,
};

/// Parses any escaped character sequence.
///
/// An escaped character sequence is a backslash character followed by any other character.
/// This parser always matches 2 characters, or fails. Note that this parser either
/// takes 2 characters or fails.
pub fn escaped_sequence<I>(input: I) -> ParseResult<I, I>
where
    I: SubsetRange<I> + SplitAt + Clone + IsEmpty + PrefixEnd<&'static str> + ItemsIndices<char>,
{
    recognize((tag("\\"), take(1))).parse(input)
}

/// Parses and consumes all spaces and tabs at the beginning of the input,
/// then verifies that the amount of whitespace is at least 4.
///
/// This space scoring technique is described in the [CommonMark spec](https://spec.commonmark.org/0.31.2/#tabs).
pub fn indented_by_at_least_4<I>(input: I) -> ParseResult<I, I>
where
    I: ItemsIndices<char> + Clone + SplitAt,
{
    validate(space_or_tab(), |s: &I| {
        s.items().any(|c| c == '\t') || s.items().count() >= 4
    })
    .parse(input)
}

/// Parses and consumes all spaces and tabs at the beginning of the input,
/// then verifies that there are no tabs and the amount of whitespace is less than 4.
///
/// Tabs are not allowed, as they count for 4 spaces. See here [CommonMark spec](https://spec.commonmark.org/0.31.2/#tabs).
pub fn indented_by_less_than_4<I>(input: I) -> ParseResult<I, I>
where
    I: ItemsIndices<char> + Clone + SplitAt,
{
    validate(space_or_tab(), |s: &I| {
        !s.items().any(|c| c == '\t') && s.items().count() < 4
    })
    .parse(input)
}

/// Consumes any amount of spaces or tabs.
///
/// If the input doesn't start with a space or a tab, the parser will succeed and
/// return an empty string as parsed.
pub fn space_or_tab<I>() -> TakeWhileParser<I, impl Fn(char) -> bool> {
    take_while(is_space_or_tab)
}

/// Consumes a line ending, which can be either `\n` or `\r\n`.
///
/// This parser will fail if the input is empty or does not start with a line ending.
pub fn line_ending<I>(input: I) -> ParseResult<I, I>
where
    I: PrefixEnd<&'static str> + SplitAt,
{
    one_of((tag("\n"), tag("\r\n"))).parse(input)
}

/// Consumes either a line ending or the end of the input.
pub fn line_ending_or_empty<I>(input: I) -> ParseResult<I, I>
where
    I: IsEmpty + PrefixEnd<&'static str> + SplitAt + Clone,
{
    one_of((line_ending, empty)).parse(input)
}

#[cfg(test)]
mod test {
    use super::*;

    mod escaped_sequence {
        use super::*;
        use crate::parse::lines;

        #[test]
        fn should_fail_with_empty_string() {
            assert!(escaped_sequence("").is_err());
        }

        #[test]
        fn should_fail_with_just_backslash() {
            assert!(escaped_sequence("\\").is_err());
        }

        #[test]
        fn should_fail_with_any_other_single_character() {
            assert!(escaped_sequence("a").is_err());
        }

        #[test]
        fn should_fail_with_an_unescaped_pair_of_characters() {
            assert!(escaped_sequence("a\\").is_err());
        }

        #[test]
        fn should_work_with_double_backslash() {
            assert_eq!(Ok(("", "\\\\")), escaped_sequence("\\\\"));
        }

        #[test]
        fn should_work_with_escaped_ascii_character() {
            assert_eq!(Ok(("", "\\a")), escaped_sequence("\\a"));
        }

        #[test]
        fn should_work_with_escaped_unicode_character() {
            assert_eq!(Ok(("", "\\é")), escaped_sequence("\\é"));
        }

        #[test]
        fn should_work_with_lines() {
            let result = escaped_sequence(lines("\\\n\ntoto"));
            assert_eq!(Ok((lines("\ntoto"), lines("\\\n"))), result);
        }
    }

    mod indented_by_at_least_4 {
        use super::*;
        use crate::parse::lines;

        #[test]
        fn should_fail_with_empty_string() {
            assert!(indented_by_at_least_4("").is_err());
        }

        #[test]
        fn should_fail_with_3_spaces() {
            assert!(indented_by_at_least_4("   ").is_err());
        }

        #[test]
        fn should_fail_with_non_whitespace_character() {
            assert!(indented_by_at_least_4("   a   ").is_err());
        }

        #[test]
        fn should_work_with_a_tab() {
            assert_eq!(Ok(("abc", "\t")), indented_by_at_least_4("\tabc"));
        }

        #[test]
        fn should_work_with_4_spaces() {
            assert_eq!(Ok(("abc", "    ")), indented_by_at_least_4("    abc"));
        }

        #[test]
        fn should_work_with_a_mix() {
            let result = indented_by_at_least_4("   \t  \t  toto");
            assert_eq!(Ok(("toto", "   \t  \t  ")), result);
        }

        #[test]
        fn should_work_with_lines() {
            let result = indented_by_at_least_4(lines("    \n    abc"));
            assert_eq!(Ok((lines("\n    abc"), lines("    "))), result);
        }
    }

    mod indented_by_less_than_4 {
        use super::*;
        use crate::parse::lines;

        #[test]
        fn should_fail_with_tab() {
            assert!(indented_by_less_than_4("\t").is_err());
        }

        #[test]
        fn should_fail_with_4_spaces() {
            assert!(indented_by_less_than_4("    ").is_err());
        }

        #[test]
        fn should_work_with_empty_string() {
            assert_eq!(Ok(("", "")), indented_by_less_than_4(""));
        }

        #[test]
        fn should_work_without_indentation() {
            assert_eq!(Ok(("abc", "")), indented_by_less_than_4("abc"));
        }

        #[test]
        fn should_work_with_3_spaces() {
            assert_eq!(Ok(("abc", "   ")), indented_by_less_than_4("   abc"));
        }

        #[test]
        fn should_work_with_lines() {
            let result = indented_by_less_than_4(lines("   toto\n   abc"));
            assert_eq!(Ok((lines("toto\n   abc"), lines("   "))), result);
        }
    }

    mod space_or_tab {
        use super::*;
        use crate::parse::lines;

        #[test]
        fn should_work_with_empty_string() {
            assert_eq!(Ok(("", "")), space_or_tab().parse(""));
        }

        #[test]
        fn should_work_when_input_does_not_start_with_space_or_tab() {
            assert_eq!(Ok(("abc", "")), space_or_tab().parse("abc"));
        }

        #[test]
        fn should_work_with_spaces_or_tabs() {
            assert_eq!(Ok(("toto", "  \t\t ")), space_or_tab().parse("  \t\t toto"));
        }

        #[test]
        fn should_work_with_lines() {
            let result = space_or_tab().parse(lines("  \t \n toto"));
            assert_eq!(Ok((lines("\n toto"), lines("  \t "))), result);
        }
    }

    mod line_ending {
        use super::*;
        use crate::parse::lines;

        #[test]
        fn should_fail_with_empty_string() {
            assert!(line_ending("").is_err());
        }

        #[test]
        fn should_fail_with_any_other_char() {
            assert!(line_ending("a").is_err());
        }

        #[test]
        fn should_work_with_crlf() {
            assert_eq!(Ok(("", "\r\n")), line_ending("\r\n"));
        }

        #[test]
        fn should_work_with_lf() {
            assert_eq!(Ok(("", "\n")), line_ending("\n"));
        }

        #[test]
        fn should_work_with_lines() {
            let result = line_ending(lines("\r\n\n"));
            assert_eq!(Ok((lines("\n"), lines("\r\n"))), result);
        }
    }

    mod line_ending_or_empty {
        use super::*;
        use crate::parse::lines;

        #[test]
        fn should_fail_with_non_empty_string() {
            assert!(line_ending_or_empty("a").is_err());
        }

        #[test]
        fn should_work_with_empty_string() {
            assert_eq!(Ok(("", "")), line_ending_or_empty(""));
        }

        #[test]
        fn should_work_with_crlf() {
            assert_eq!(Ok(("", "\r\n")), line_ending_or_empty("\r\n"));
        }

        #[test]
        fn should_work_with_lf() {
            assert_eq!(Ok(("", "\n")), line_ending_or_empty("\n"));
        }

        #[test]
        fn should_work_with_lines() {
            let result = line_ending_or_empty(lines(""));
            assert_eq!(Ok((lines(""), lines(""))), result);
        }
    }
}
