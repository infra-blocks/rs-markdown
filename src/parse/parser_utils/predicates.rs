use crate::{
    ast::block::BlankLine,
    parse::{parser::is_one_of, traits::ParseLine},
};

/// Returns whether a character is a space or tab.
pub fn is_space_or_tab(c: char) -> bool {
    is_one_of(&[' ', '\t'])(c)
}

/// Returns whether the input is a blank line or not.
pub fn is_blank_line(input: &str) -> bool {
    BlankLine::parse_line(input).is_ok()
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
        fn should_return_false_for_empty_string() {
            assert!(!is_blank_line(""));
        }

        #[test]
        fn should_return_false_if_contains_non_blank_characters() {
            assert!(!is_blank_line(" \tword\n"));
        }

        #[test]
        fn should_return_true_for_blank_line() {
            assert!(is_blank_line(" \n"));
            assert!(is_blank_line("\t\n"));
            assert!(is_blank_line(" \t\n"));
        }
    }
}
