use crate::{
    ast::inline::link::LinkLabel,
    parse::{parsers::escaped_sequence, traits::ParseLine},
};
use parser::{
    Map, ParseResult, Parser, Repeated, is_one_of, not, one_of, recognize, tag, take_while,
    validate,
};

/*
From the spec:
A link label begins with a left bracket ([) and ends with the first right bracket (]) that is not backslash-escaped.
Between these brackets there must be at least one character that is not a space, tab, or line ending.
Unescaped square bracket characters are not allowed inside the opening and closing square brackets of link labels.
A link label can have at most 999 characters inside the square brackets.
*/
impl<'a> ParseLine<'a> for LinkLabel<'a> {
    fn parse_line(input: &'a str) -> ParseResult<&'a str, Self> {
        recognize((
            tag("["),
            validate(
                recognize(
                    (one_of((
                        escaped_sequence,
                        take_while(not(is_one_of(&['\\', '[', ']']))).at_least(1),
                    )))
                    .repeated()
                    .at_least(1),
                ),
                |segment: &&str| utils::is_valid_content(segment),
            ),
            tag("]"),
        ))
        .map(Self::new)
        .parse(input)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    mod parse {
        use super::*;
        use crate::parse::test_utils::test_parse_macros;

        test_parse_macros!(LinkLabel);

        failure_case!(should_reject_empty_segment, "");
        failure_case!(should_reject_blank_line, "\n");
        failure_case!(should_reject_missing_closing_bracket, "[a");
        failure_case!(should_reject_missing_opening_backet, "a]");
        failure_case!(should_reject_leading_whitespace, " [a]");
        failure_case!(should_reject_empty_link_label, "[]");
        failure_case!(should_reject_whitespace_link_label, "[ \t ]");
        failure_case!(should_reject_multiple_opening_brackets, "[[a]");
        failure_case!(
            should_reject_verbose_label,
            concat!(
                "[",
                "Lorem ipsum dolor sit amet, consectetuer adipiscing elit. Aenean commodo ligula eget dolor. Aenean m",
                "assa. Cum sociis natoque penatibus et magnis dis parturient montes, nascetur ridiculus mus. Donec qu",
                "am felis, ultricies nec, pellentesque eu, pretium quis, sem. Nulla consequat massa quis enim. Donec ",
                "pede justo, fringilla vel, aliquet nec, vulputate eget, arcu. In enim justo, rhoncus ut, imperdiet a",
                ", venenatis vitae, justo. Nullam dictum felis eu pede mollis pretium. Integer tincidunt. Cras dapibu",
                "s. Vivamus elementum semper nisi. Aenean vulputate eleifend tellus. Aenean leo ligula, porttitor eu,",
                " consequat vitae, eleifend ac, enim. Aliquam lorem ante, dapibus in, viverra quis, feugiat a, tellus",
                ". Phasellus viverra nulla ut metus varius laoreet. Quisque rutrum. Aenean imperdiet. Etiam ultricies",
                " nisi vel augue. Curabitur ullamcorper ultricies nisi. Nam eget dui. Etiam rhoncus. Maecenas tempus,",
                " tellus eget condimentum rhoncus, sem quam semper libero, sit amet adipiscing sem neque sed ipsum. N",
                "]"
            )
        );

        success_case!(should_work_with_a_simple_link_label, "[a]");
        success_case!(should_work_with_included_whitespace, "[ a ]");
        success_case!(should_work_with_double_backslash, r"[\\]");
        success_case!(should_work_with_escaped_closing_bracket, r"[\]]");
        success_case!(should_work_with_escaped_opening_bracket, r"[\[]");
        success_case!(should_work_with_several_words, "[a b c]");
        success_case!(
            should_work_with_999_characters,
            concat!(
                "[",
                "Lorem ipsum dolor sit amet, consectetuer adipiscing elit. Aenean commodo ligula eget dolor. Aenean m",
                "assa. Cum sociis natoque penatibus et magnis dis parturient montes, nascetur ridiculus mus. Donec qu",
                "am felis, ultricies nec, pellentesque eu, pretium quis, sem. Nulla consequat massa quis enim. Donec ",
                "pede justo, fringilla vel, aliquet nec, vulputate eget, arcu. In enim justo, rhoncus ut, imperdiet a",
                ", venenatis vitae, justo. Nullam dictum felis eu pede mollis pretium. Integer tincidunt. Cras dapibu",
                "s. Vivamus elementum semper nisi. Aenean vulputate eleifend tellus. Aenean leo ligula, porttitor eu,",
                " consequat vitae, eleifend ac, enim. Aliquam lorem ante, dapibus in, viverra quis, feugiat a, tellus",
                ". Phasellus viverra nulla ut metus varius laoreet. Quisque rutrum. Aenean imperdiet. Etiam ultricies",
                " nisi vel augue. Curabitur ullamcorper ultricies nisi. Nam eget dui. Etiam rhoncus. Maecenas tempus,",
                " tellus eget condimentum rhoncus, sem quam semper libero, sit amet adipiscing sem neque sed ipsum. ",
                "]"
            )
        );
        success_case!(should_stop_after_closing_bracket, "[a] ", "[a]", " ");
    }
}

mod utils {
    pub fn is_valid_content(segment: &str) -> bool {
        // Check if the segment contains at least one non-whitespace character
        !segment.trim().is_empty() && valid_character_count(segment)
    }

    /// A link label can have at most 999 characters inside the square brackets.
    pub fn valid_character_count(segment: &str) -> bool {
        segment.chars().count() <= 999
    }
}
