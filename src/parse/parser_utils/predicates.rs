use crate::{ast::block::BlankLine, parse::traits::ParseLine};
use parser::is_one_of;

/// Returns whether a character is a space or tab.
pub fn is_space_or_tab(c: char) -> bool {
    is_one_of(&[' ', '\t'])(c)
}

/// Returns whether the input is a blank line or not.
pub fn is_blank_line(input: &str) -> bool {
    match BlankLine::parse_line(input) {
        Ok((remaining, _)) => remaining.is_empty(),
        Err(_) => false,
    }
}

#[cfg(test)]
mod test {
    use super::*;

    mod is_space_or_tab {
        use super::*;

        #[test]
        fn should_return_false_for_other_characters() {
            assert!(!is_space_or_tab('a'));
            assert!(!is_space_or_tab('\n'));
            assert!(!is_space_or_tab('1'));
        }

        #[test]
        fn should_return_true_for_space() {
            assert!(is_space_or_tab(' '));
        }

        #[test]
        fn should_return_true_for_tab() {
            assert!(is_space_or_tab('\t'));
        }
    }

    mod is_blank_line {
        use super::*;

        #[test]
        fn should_return_false_with_empty_string() {
            assert!(!is_blank_line(""));
        }

        #[test]
        fn should_return_false_for_string_with_one_none_whitespace_character() {
            assert!(!is_blank_line(" \ta\n"));
        }

        #[test]
        fn should_return_false_for_2_blank_lines() {
            assert!(!is_blank_line("\n\n"));
        }

        #[test]
        fn should_return_false_for_a_blank_line_followed_by_a_character() {
            assert!(!is_blank_line("\nabc"));
        }

        #[test]
        fn should_return_true_with_space() {
            assert!(is_blank_line(" "));
        }

        #[test]
        fn should_return_true_with_tab() {
            assert!(is_blank_line("\t"));
        }

        #[test]
        fn should_return_true_for_carriage_return() {
            assert!(is_blank_line("\r\n"));
        }

        #[test]
        fn shoud_return_true_for_newline() {
            assert!(is_blank_line("\n"));
        }
    }
}
