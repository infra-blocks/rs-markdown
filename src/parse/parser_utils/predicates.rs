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

/// Returns whether the parentheses in the segment are balanced.
///
/// Escaped parentheses are ignored.
pub fn parentheseses_balance(segment: &str) -> bool {
    // Ignore escaped parentheseses by removing them.
    let sanitized = segment.replace(r"\(", "").replace(r"\)", "");
    // Ensure the count of opening and closing parentheseses is equal.
    sanitized.chars().filter(|&c| c == '(').count()
        == sanitized.chars().filter(|&c| c == ')').count()
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

    mod parentheseses_balance {
        use super::*;

        #[test]
        fn should_reject_single_opening_parenthesis() {
            assert!(!parentheseses_balance("("));
        }

        #[test]
        fn should_reject_single_closing_parenthesis() {
            assert!(!parentheseses_balance(")"));
        }

        #[test]
        fn should_reject_unbalanced_parentheses() {
            assert!(!parentheseses_balance("(foo(and(bar))"));
        }

        #[test]
        fn should_accept_an_empty_string() {
            assert!(parentheseses_balance(""));
        }

        #[test]
        fn should_accept_string_without_parentheses() {
            assert!(parentheseses_balance("foo"));
        }

        #[test]
        fn should_accept_unbalanced_escaped_parentheses() {
            assert!(parentheseses_balance(r"\(\(foo\)and\(bar\)"));
        }

        #[test]
        fn should_accept_balanced_parentheses() {
            assert!(parentheseses_balance("(foo(and(bar)))"));
        }

        #[test]
        fn should_accept_balanced_parentheses_and_ignore_escaped_ones() {
            assert!(parentheseses_balance(r"(foo\(blip(and(bar)))"));
        }
    }
}
