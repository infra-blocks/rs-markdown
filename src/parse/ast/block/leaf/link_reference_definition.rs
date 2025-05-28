use crate::{
    ast::{
        block::LinkReferenceDefinition,
        inline::link::{LinkDestination, LinkLabel, LinkTitle},
    },
    parse::{
        input::Input,
        parsers::{indented_by_less_than_4, line_ending, line_ending_or_empty, space_or_tab},
        traits::Parse,
    },
};
use parser::{
    ItemsIndices, Map, ParseResult, Parser, consumed, maybe, one_of, recognize, tag, validate,
};

/// Parses the link label in the context of the link reference definition.
///
/// It consumes the leading indentation (if any), the label itself by dispatching to [LinkLabel::parse],
/// then the following colon.
fn label<'a, I: Input<'a>>(input: I) -> ParseResult<I, LinkLabel<'a>> {
    (indented_by_less_than_4, LinkLabel::parse, tag(":"))
        .map(|(_, label, _)| label)
        .parse(input)
}

/// Parses the destination right where the [label] parser left off.
///
/// Consumes all spaces or tabs, and optionally a line ending, then dispatches to [LinkDestination::parse].
fn destination<'a, I: Input<'a>>(input: I) -> ParseResult<I, LinkDestination<'a>> {
    (
        space_or_tab(),
        maybe(line_ending),
        space_or_tab(),
        LinkDestination::parse,
    )
        .map(|(_, _, _, destination)| destination)
        .parse(input)
}

/// Parses the optional title of the link reference definition.
///
/// This parser either parses a valid title or fails. A valid title needs to be separated
/// by at least one space from the destination, can include one line ending in between,
/// and can only be followed by whitespaces until the end of the line or end of the input.
/// If any of those things is not satisfied, the parser will fail.
fn title<'a, I: Input<'a>>(input: I) -> ParseResult<I, LinkTitle<'a>> {
    (
        validate(
            recognize((space_or_tab(), maybe(line_ending), space_or_tab())),
            |parsed: &I| !parsed.is_empty(),
        ),
        LinkTitle::parse,
        // No further character may occur after the title, if there.
        space_or_tab(),
        line_ending_or_empty,
    )
        .map(|(_, title, _, _)| title)
        .parse(input)
}

/// Parses the final segment(s) of the link reference definition taking over where [destination] left off.
///
/// The remaining input can either be a valid title, as parsed by [title], or it can be white spaces
/// until the end of input or the end of the line.
fn title_or_end<'a, I: Input<'a>>(input: I) -> ParseResult<I, Option<LinkTitle<'a>>> {
    one_of((
        title.map(Some),
        (space_or_tab(), line_ending_or_empty).map(|_| None),
    ))
    .parse(input)
}

impl<'a> Parse<'a> for LinkReferenceDefinition<'a> {
    fn parse<I: Input<'a>>(input: I) -> ParseResult<I, Self> {
        consumed((label, destination, title_or_end))
            .map(|(consumed, (label, destination, title))| {
                LinkReferenceDefinition::new(
                    <I as ItemsIndices<&'a str>>::items(&consumed).collect(),
                    label,
                    destination,
                    title,
                )
            })
            .parse(input)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    mod parse {
        use super::*;
        use crate::parse::test_utils::{StrictParse, test_parse_macros};

        test_parse_macros!(LinkReferenceDefinition);

        failure_case!(should_reject_empty, "");
        failure_case!(should_reject_blank_line, "\n");
        failure_case!(should_reject_missing_label, "/url 'title'");
        failure_case!(should_reject_missing_destination, "[foo]:");
        failure_case!(
            should_reject_characters_after_title,
            "[foo]: /url 'title'facalacab"
        );

        success_case!(should_work_with_without_title, "[foo]: /url", parsed => LinkReferenceDefinition::new(
            vec!["[foo]: /url"],
            LinkLabel::strict_parse("[foo]"),
            LinkDestination::strict_parse("/url"),
            None
        ));
        success_case!(should_work_with_everything_on_one_line, "[foo]: /url 'title'", parsed => LinkReferenceDefinition::new(
            vec!["[foo]: /url 'title'"],
            LinkLabel::strict_parse("[foo]"),
            LinkDestination::strict_parse("/url"),
            Some(LinkTitle::strict_parse("'title'"))
        ));
        success_case!(should_work_with_newline_between_label_and_destination, "[foo]:\n      /url", parsed => LinkReferenceDefinition::new(
            vec!["[foo]:\n", "      /url"],
            LinkLabel::strict_parse("[foo]"),
            LinkDestination::strict_parse("/url"),
            None
        ));
        success_case!(should_work_with_each_on_different_line, "[Foo bar]:\n</url>\n'title'\n", parsed => LinkReferenceDefinition::new(
            vec!["[Foo bar]:\n", "</url>\n", "'title'\n"],
            LinkLabel::strict_parse("[Foo bar]"),
            LinkDestination::strict_parse("</url>"),
            Some(LinkTitle::strict_parse("'title'"))
        ));
        success_case!(should_work_with_title_that_extends_multiple_lines, "[foo]: /url '\ntitle\nline1\nline2\n'\ntoto", parsed => LinkReferenceDefinition::new(
            vec!["[foo]: /url '\n", "title\n", "line1\n", "line2\n", "'\n"],
            LinkLabel::strict_parse("[foo]"),
            LinkDestination::strict_parse("/url"),
            Some(LinkTitle::strict_parse("'\ntitle\nline1\nline2\n'"))
        ), "toto");
    }
}
