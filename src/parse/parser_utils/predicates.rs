use crate::parse::parser::is_one_of;

/// Returns whether a character is a space or tab.
pub fn is_space_or_tab(c: char) -> bool {
    is_one_of(&[' ', '\t'])(c)
}
