use super::is_space_or_tab;
use parser::{
    ParseResult, Parser, TakeWhileParser, one_of, recognize, tag, take, take_while, validate,
};

/// Parses any escaped character sequence.
///
/// An escaped character sequence is a backslash character followed by any other character.
/// This parser always matches 2 characters, or fails. Note that this parser either
/// takes 2 characters or fails.
pub fn escaped_sequence(input: &str) -> ParseResult<&str, &str> {
    recognize((tag("\\"), take(1))).parse(input)
}

/// Parses and consumes all spaces and tabs at the beginning of the input,
/// then verifies that the amount of whitespace is at least 4.
///
/// This space scoring technique is described in the [CommonMark spec](https://spec.commonmark.org/0.31.2/#tabs).
pub fn indented_by_at_least_4(input: &str) -> ParseResult<&str, &str> {
    validate(space_or_tab(), |s: &&str| s.contains("\t") || s.len() >= 4).parse(input)
}

/// Parses and consumes all spaces and tabs at the beginning of the input,
/// then verifies that there are no tabs and the amount of whitespace is less than 4.
///
/// Tabs are not allowed, as they count for 4 spaces. See here [CommonMark spec](https://spec.commonmark.org/0.31.2/#tabs).
pub fn indented_by_less_than_4(input: &str) -> ParseResult<&str, &str> {
    validate(space_or_tab(), |s: &&str| !s.contains("\t") && s.len() < 4).parse(input)
}

/// Consumes any amount of spaces or tabs.
///
/// If the input doesn't start with a space or a tab, the parser will succeed and
/// return an empty string as parsed.
pub fn space_or_tab<E>() -> TakeWhileParser<E, impl Fn(char) -> bool> {
    take_while(is_space_or_tab)
}

/// Consumes a line ending, which can be either `\n` or `\r\n`.
///
/// This parser will fail if the input is empty or does not start with a line ending.
pub fn line_ending(input: &str) -> ParseResult<&str, &str> {
    one_of((tag("\n"), tag("\r\n"))).parse(input)
}

/// Consumes an empty string, signifying the end of the input.
///
/// This parser will fail if the input is not empty.
pub fn eof(input: &str) -> ParseResult<&str, &str> {
    if input.is_empty() {
        Ok(("", ""))
    } else {
        Err(input)
    }
}

/// Consumes either a line ending or the end of the input.
pub fn line_ending_or_eof(input: &str) -> ParseResult<&str, &str> {
    one_of((line_ending, eof)).parse(input)
}

#[cfg(test)]
mod test {
    use super::*;

    mod escaped_sequence {
        use super::*;

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
            let (remaining, parsed) = escaped_sequence("\\\\").unwrap();
            assert_eq!(remaining, "");
            assert_eq!(parsed, "\\\\");
        }

        #[test]
        fn should_work_with_escaped_ascii_character() {
            let (remaining, parsed) = escaped_sequence("\\a").unwrap();
            assert_eq!(remaining, "");
            assert_eq!(parsed, "\\a");
        }

        #[test]
        fn should_work_with_escaped_unicode_character() {
            let (remaining, parsed) = escaped_sequence("\\é").unwrap();
            assert_eq!(remaining, "");
            assert_eq!(parsed, "\\é");
        }
    }

    mod indented_by_at_least_4 {
        use super::*;

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
            let (remaining, parsed) = indented_by_at_least_4("\tabc").unwrap();
            assert_eq!(remaining, "abc");
            assert_eq!(parsed, "\t");
        }

        #[test]
        fn should_work_with_4_spaces() {
            let (remaining, parsed) = indented_by_at_least_4("    abc").unwrap();
            assert_eq!(remaining, "abc");
            assert_eq!(parsed, "    ");
        }

        #[test]
        fn should_work_with_a_mix() {
            let (remaining, parsed) = indented_by_at_least_4("   \t  \t  toto").unwrap();
            assert_eq!(remaining, "toto");
            assert_eq!(parsed, "   \t  \t  ");
        }
    }

    mod indented_by_less_than_4 {
        use super::*;

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
    }

    mod space_or_tab {
        use super::*;

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
    }

    mod line_ending {
        use super::*;

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
    }

    mod eof {
        use super::*;

        #[test]
        fn should_fail_with_non_empty_string() {
            assert!(eof("a").is_err());
        }

        #[test]
        fn should_work_with_empty_string() {
            assert_eq!(Ok(("", "")), eof(""));
        }
    }

    mod line_ending_or_eof {
        use super::*;

        #[test]
        fn should_fail_with_non_empty_string() {
            assert!(line_ending_or_eof("a").is_err());
        }

        #[test]
        fn should_work_with_empty_string() {
            assert_eq!(Ok(("", "")), line_ending_or_eof(""));
        }

        #[test]
        fn should_work_with_crlf() {
            assert_eq!(Ok(("", "\r\n")), line_ending_or_eof("\r\n"));
        }

        #[test]
        fn should_work_with_lf() {
            assert_eq!(Ok(("", "\n")), line_ending_or_eof("\n"));
        }
    }
}
