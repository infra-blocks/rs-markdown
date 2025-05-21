use super::is_space_or_tab;
use crate::parse::parser::{ParseResult, Parser, one_of, tag, take_while, validate};

/// Parses the leading indentation of a line, up to 3 spaces.
///
/// Tabs are not allowed, as they count for 4 spaces. See here [CommonMark spec](https://spec.commonmark.org/0.31.2/#tabs).
pub fn indented_by_less_than_4(input: &str) -> ParseResult<&str, &str> {
    validate(take_while(is_space_or_tab), |spaces: &&str| {
        !spaces.contains("\t") && spaces.len() < 4
    })
    .parse(input)
}

/// Consumes any amount of spaces or tabs.
///
/// If the input doesn't start with a space or a tab, the parser will succeed and
/// return an empty string as parsed.
pub fn space_or_tab(input: &str) -> ParseResult<&str, &str> {
    take_while(is_space_or_tab).parse(input)
}

pub fn line_ending(input: &str) -> ParseResult<&str, &str> {
    one_of((tag("\n"), tag("\r\n"))).parse(input)
}

pub fn eof(input: &str) -> ParseResult<&str, &str> {
    if input.is_empty() {
        Ok(("", ""))
    } else {
        Err(input)
    }
}

pub fn line_ending_or_eof(input: &str) -> ParseResult<&str, &str> {
    one_of((line_ending, eof)).parse(input)
}

#[cfg(test)]
mod test {
    use super::*;

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
            assert_eq!(Ok(("", "")), space_or_tab(""));
        }

        #[test]
        fn should_work_when_input_does_not_start_with_space_or_tab() {
            assert_eq!(Ok(("abc", "")), space_or_tab("abc"));
        }

        #[test]
        fn should_work_with_spaces_or_tabs() {
            assert_eq!(Ok(("toto", "  \t\t ")), space_or_tab("  \t\t toto"));
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
