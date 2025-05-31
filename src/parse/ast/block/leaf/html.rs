use crate::{
    ast::block::Html,
    parse::{
        input::Input,
        parsers::{indented_by_less_than_4, line_ending_or_empty, space_or_tab},
        predicates::is_blank_line,
        traits::Parse,
    },
};
use parser::{
    Map, ParseResult, Parser, any_tag, empty, is, is_one_of, maybe, one_of, recognize, rest, tag,
    take, take_while, validate,
};
use std::result;

const CASE_1_TAG_NAMES: [&str; 4] = ["pre", "script", "style", "textarea"];
const CASE_6_TAG_NAMES: [&str; 62] = [
    "address",
    "article",
    "aside",
    "base",
    "basefont",
    "blockquote",
    "body",
    "caption",
    "center",
    "col",
    "colgroup",
    "dd",
    "details",
    "dialog",
    "dir",
    "div",
    "dl",
    "dt",
    "fieldset",
    "figcaption",
    "figure",
    "footer",
    "form",
    "frame",
    "frameset",
    "h1",
    "h2",
    "h3",
    "h4",
    "h5",
    "h6",
    "head",
    "header",
    "hr",
    "html",
    "iframe",
    "legend",
    "li",
    "link",
    "main",
    "menu",
    "menuitem",
    "nav",
    "noframes",
    "ol",
    "optgroup",
    "option",
    "p",
    "param",
    "search",
    "section",
    "summary",
    "table",
    "tbody",
    "td",
    "tfoot",
    "th",
    "thead",
    "title",
    "tr",
    "track",
    "ul",
];

impl<'a> Parse<'a> for Html<'a> {
    fn parse<I: Input<'a>>(input: I) -> ParseResult<I, Self> {
        recognize((
            indented_by_less_than_4,
            one_of((case_1, case_2, case_3, case_4, case_5, case_6, case_7)),
        ))
        .map(|parsed: I| Html::new(parsed.lines().collect()))
        .parse(input)
    }
}

/// This is case 1 in the spec, and covers lines with the following
/// - Start condition: line begins with the string <pre, <script, <style, or <textarea (case-insensitive),
///   followed by a space, a tab, the string >, or the end of the line.
/// - End condition: line contains an end tag </pre>, </script>, </style>, or </textarea> (case-insensitive; it need not match the start tag).
fn case_1<'a, I: Input<'a>>(input: I) -> ParseResult<I, I> {
    let start = |s: &str| is(case_1_opening_segment)(s);
    let end = |s: &str| is(case_1_closing_segment)(s);
    utils::within_conditions(start, end).parse(input)
}

pub fn case_1_opening_segment(input: &str) -> ParseResult<&str, &str> {
    recognize((
        tag("<"),
        validate(utils::tag_name, |tag_name: &&str| {
            CASE_1_TAG_NAMES
                .iter()
                .any(|name| name.eq_ignore_ascii_case(tag_name))
        }),
        one_of((take(1).that(is_one_of(&[' ', '\t', '\n', '>'])), empty)),
    ))
    .parse(input)
}

fn case_1_closing_segment(input: &str) -> ParseResult<&str, &str> {
    // This is a helper function to check if the input is a valid end condition for case 1.
    let s = input.to_lowercase();
    if s.contains("</pre>")
        || s.contains("</script>")
        || s.contains("</style>")
        || s.contains("</textarea>")
    {
        rest(input)
    } else {
        Err(input)
    }
}

/// This is case 2 in the spec, and covers lines with the following
/// start and end conditions:
/// - Start condition: line begins with the string <!--.
/// - End condition: line contains the string -->.
fn case_2<'a, I: Input<'a>>(input: I) -> ParseResult<I, I> {
    let start = |s: &str| is(case_2_opening_segment)(s);
    let end = |s: &str| is(case_2_closing_segment)(s);
    utils::within_conditions(start, end).parse(input)
}

pub fn case_2_opening_segment(input: &str) -> ParseResult<&str, &str> {
    if input.starts_with("<!--") {
        rest(input)
    } else {
        Err(input)
    }
}

fn case_2_closing_segment(input: &str) -> ParseResult<&str, &str> {
    if input.contains("-->") {
        rest(input)
    } else {
        Err(input)
    }
}

/// This is case 3 in the spec, and covers lines with the following
/// start and end conditions:
/// - Start condition: line begins with the string <?.
/// - End condition: line contains the string ?>.
fn case_3<'a, I: Input<'a>>(input: I) -> ParseResult<I, I> {
    let start = |s: &str| is(case_3_opening_segment)(s);
    let end = |s: &str| is(case_3_closing_segment)(s);
    utils::within_conditions(start, end).parse(input)
}

pub fn case_3_opening_segment(input: &str) -> ParseResult<&str, &str> {
    if input.starts_with("<?") {
        rest(input)
    } else {
        Err(input)
    }
}

fn case_3_closing_segment(input: &str) -> ParseResult<&str, &str> {
    if input.contains("?>") {
        rest(input)
    } else {
        Err(input)
    }
}

/// This is case 4 in the spec, and covers lines with the following
/// start and end conditions:
/// - Start condition: line begins with the string <! followed by an ASCII letter.
/// - End condition: line contains the string >.
fn case_4<'a, I: Input<'a>>(input: I) -> ParseResult<I, I> {
    let start = |s: &str| is(case_4_opening_segment)(s);
    let end = |s: &str| is(case_4_closing_segment)(s);
    utils::within_conditions(start, end).parse(input)
}

pub fn case_4_opening_segment(input: &str) -> ParseResult<&str, &str> {
    if input.starts_with("<!")
        && input
            .chars()
            .nth(2)
            .map_or(false, |c| c.is_ascii_alphabetic())
    {
        rest(input)
    } else {
        Err(input)
    }
}

fn case_4_closing_segment(input: &str) -> ParseResult<&str, &str> {
    if input.contains('>') {
        rest(input)
    } else {
        Err(input)
    }
}

/// This is case 5 in the spec, and covers lines with the following
/// start and end conditions:
/// - Start condition: line begins with the string <![CDATA[.
/// - End condition: line contains the string ]]>.
fn case_5<'a, I: Input<'a>>(input: I) -> ParseResult<I, I> {
    let start = |s: &str| is(case_5_opening_segment)(s);
    let end = |s: &str| is(case_5_closing_segment)(s);
    utils::within_conditions(start, end).parse(input)
}

pub fn case_5_opening_segment(input: &str) -> ParseResult<&str, &str> {
    if input.starts_with("<![CDATA[") {
        rest(input)
    } else {
        Err(input)
    }
}

fn case_5_closing_segment(input: &str) -> ParseResult<&str, &str> {
    if input.contains("]]>") {
        rest(input)
    } else {
        Err(input)
    }
}

/// This is case 6 in the spec, and covers lines with the following
/// start and end conditions:
/// - Start condition: line begins with the string < or </ followed by one of the strings (case-insensitive) address,
///   article, aside, base, basefont, blockquote, body, caption, center, col, colgroup, dd, details, dialog, dir,
///   div, dl, dt, fieldset, figcaption, figure, footer, form, frame, frameset, h1, h2, h3, h4, h5, h6, head, header,
///   hr, html, iframe, legend, li, link, main, menu, menuitem, nav, noframes, ol, optgroup, option, p, param, search,
///   section, summary, table, tbody, td, tfoot, th, thead, title, tr, track, ul, followed by a space, a tab,
///   the end of the line, the string >, or the string />.
/// - End condition: line is followed by a blank line.
fn case_6<'a, I: Input<'a>>(input: I) -> ParseResult<I, I> {
    let start = |s: &str| is(case_6_opening_segment)(s);
    // Because this particular case needs to check the next line, we don't reuse the `within_conditions` function.
    recognize((take(1).that(start), take_while(|s: &str| !is_blank_line(s)))).parse(input)
}

pub fn case_6_opening_segment(input: &str) -> ParseResult<&str, &str> {
    recognize((
        (
            tag("<"),
            maybe(tag("/")),
            validate(utils::tag_name, |tag_name: &&str| {
                CASE_6_TAG_NAMES
                    .iter()
                    .any(|name| name.eq_ignore_ascii_case(tag_name))
            }),
        ),
        one_of((
            take(1).that(is_one_of(&[' ', '\t', '\n', '>'])),
            tag("/>"),
            empty,
        )),
    ))
    .parse(input)
}

/// This is case 7 in the spec, and covers lines with the following
/// start and end conditions:
/// - Start condition: line begins with a complete open tag (with any tag name other than pre, script, style,
///   or textarea) or a complete closing tag, followed by zero or more spaces and tabs, followed by the end of
///   the line.
/// - End condition: line is followed by a blank line.
fn case_7<'a, I: Input<'a>>(input: I) -> ParseResult<I, I> {
    let tag_name_validator = |tag_name: &I| {
        // The tag name must not be one of the CASE_1_TAG_NAMES.
        // Because we know the tag name cannot contain newlines, we can simply pick the first
        // line of the input.
        let first_line = tag_name.lines().next().expect("unexpected empty tag name");
        !CASE_1_TAG_NAMES
            .iter()
            .any(|&name| name.eq_ignore_ascii_case(first_line))
    };

    recognize((
        one_of((
            utils::open_tag(tag_name_validator),
            utils::closing_tag(tag_name_validator),
        )),
        space_or_tab(),
        line_ending_or_empty,
        take_while(|s: &str| !is_blank_line(s)),
    ))
    .parse(input)
}

mod utils {
    use crate::parse::{input::Input, parsers::space_or_tab_and_up_to_1_line_ending};
    use parser::{
        IsEmpty, ItemsIndices, ParseResult, Parser, SplitAt, SubsetRange, is_one_of, maybe, not,
        one_of, recognize, repeated, tag, take, take_while, validate,
    };

    pub fn within_conditions<'a, I, S, E>(start: S, end: E) -> impl Parser<I, Output = I>
    where
        I: Input<'a>,
        S: Fn(&str) -> bool + Clone,
        E: Fn(&str) -> bool + Clone,
    {
        one_of((
            single_line(start.clone(), end.clone()),
            multi_line(start, end),
        ))
    }

    fn single_line<'a, I, S, E>(start: S, end: E) -> impl Parser<I, Output = I>
    where
        I: Input<'a>,
        S: Fn(&str) -> bool,
        E: Fn(&str) -> bool,
    {
        take(1).that(move |s: &str| start(s) && end(s))
    }

    fn multi_line<'a, I, S, E>(start: S, end: E) -> impl Parser<I, Output = I>
    where
        I: Input<'a>,
        S: Fn(&str) -> bool,
        E: Fn(&str) -> bool,
    {
        recognize((
            take(1).that(start),
            take_while(move |s: &str| !end(s)),
            // Either the previous line matched the end predicate, or we ran out of input.
            maybe(take::<&str>(1)),
        ))
    }

    /// An open tag consists of a < character, a [tag name](tag_name), zero or more attributes,
    /// optional spaces, tabs, and up to one line ending, an optional / character, and a > character.
    pub fn open_tag<'a, I: Input<'a>, F: Fn(&I) -> bool>(
        tag_name_validator: F,
    ) -> impl Parser<I, Output = I> {
        // This is a helper function to parse an open tag.
        // It is used in the case 6 parser.
        recognize((
            tag("<"),
            validate(tag_name, tag_name_validator),
            repeated(attribute),
            space_or_tab_and_up_to_1_line_ending,
            maybe(tag("/")),
            tag(">"),
        ))
    }

    /// A closing tag consists of the string </, a tag name, optional spaces, tabs, and up to one line ending, and the character >.
    pub fn closing_tag<'a, I: Input<'a>, F: Fn(&I) -> bool>(
        tag_name_validator: F,
    ) -> impl Parser<I, Output = I> {
        recognize((
            tag("</"),
            validate(tag_name, tag_name_validator),
            space_or_tab_and_up_to_1_line_ending,
            tag(">"),
        ))
    }

    /// An attribute consists of spaces, tabs, and up to one line ending, an [attribute name](attribute_name),
    /// and an optional [attribute value specification](attribute_value_specification).
    fn attribute<'a, I: Input<'a>>(input: I) -> ParseResult<I, I> {
        recognize((
            validate(space_or_tab_and_up_to_1_line_ending, |s: &I| !s.is_empty()),
            attribute_name,
            maybe(attribute_value_specification),
        ))
        .parse(input)
    }

    /// An attribute name consists of an ASCII letter, _, or :, followed by zero or more ASCII letters, digits, _, ., :, or -.
    /// (Note: This is the XML specification restricted to ASCII. HTML5 is laxer.)
    fn attribute_name<'a, I: Input<'a>>(input: I) -> ParseResult<I, I> {
        recognize((
            take(1).that(|c: char| c.is_ascii_alphabetic() || c == '_' || c == ':'),
            take_while(|c: char| {
                c.is_ascii_alphanumeric()
                    || c.is_ascii_digit()
                    || c == '-'
                    || c == '_'
                    || c == ':'
                    || c == '.'
            }),
        ))
        .parse(input)
    }

    /// An attribute value specification consists of optional spaces, tabs, and up to one line ending,
    /// a = character, optional spaces, tabs, and up to one line ending, and an [attribute value](attribute_value).
    fn attribute_value_specification<'a, I: Input<'a>>(input: I) -> ParseResult<I, I> {
        recognize((
            space_or_tab_and_up_to_1_line_ending,
            tag("="),
            space_or_tab_and_up_to_1_line_ending,
            attribute_value,
        ))
        .parse(input)
    }

    /// An attribute value consists of an [unquoted attribute value](unquoted_attribute_value),
    /// a [single-quoted attribute value](single_quoted_attribute_value), or a
    /// [double-quoted attribute value](double_quoted_attribute_value).
    fn attribute_value<'a, I: Input<'a>>(input: I) -> ParseResult<I, I> {
        one_of((
            unquoted_attribute_value,
            single_quoted_attribute_value,
            double_quoted_attribute_value,
        ))
        .parse(input)
    }

    /// An unquoted attribute value is a nonempty string of characters not including spaces,
    /// tabs, line endings, ", ', =, <, >, or `.
    fn unquoted_attribute_value<'a, I: Input<'a>>(input: I) -> ParseResult<I, I> {
        recognize(validate(
            take_while(not(is_one_of(&[
                ' ', '\t', '\r', '\n', '"', '\'', '=', '<', '>', '`',
            ]))),
            |s: &I| !s.is_empty(),
        ))
        .parse(input)
    }

    /// A single-quoted attribute value consists of ', zero or more characters not including ', and a final '.
    fn single_quoted_attribute_value<'a, I: Input<'a>>(input: I) -> ParseResult<I, I> {
        recognize((tag("'"), take_while(|c: char| c != '\''), tag("'"))).parse(input)
    }

    /// A double-quoted attribute value consists of ", zero or more characters not including ", and a final "
    fn double_quoted_attribute_value<'a, I: Input<'a>>(input: I) -> ParseResult<I, I> {
        recognize((tag("\""), take_while(|c: char| c != '"'), tag("\""))).parse(input)
    }

    /// A tag name consists of an ASCII letter followed by zero or more ASCII letters, digits, or hyphens (-).
    pub fn tag_name<I>(input: I) -> ParseResult<I, I>
    where
        I: SubsetRange<I> + SplitAt + Clone + IsEmpty + ItemsIndices<char>,
    {
        recognize((
            take(1).that(|c: char| c.is_ascii_alphabetic()),
            take_while(|c: char| c.is_ascii_alphanumeric() || c.is_ascii_digit() || c == '-'),
        ))
        .parse(input)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    mod parse {
        use super::*;
        use crate::parse::test_utils::test_parse_macros;

        test_parse_macros!(Html);

        failure_case!(should_reject_empty, "");
        failure_case!(should_reject_a_blank_line, "\n");
        failure_case!(
            should_reject_4_spaces_of_indentation,
            "    <div> content </div>\n"
        );
        failure_case!(should_reject_tab_indentation, "\t<div> content </div>\n");

        mod case_1 {
            use super::*;

            // We test the fact that the end tag does not need to match the start tag
            // and that the parser is case-insensitive all throughout.
            success_case!(
                should_work_with_single_line,
                "<pre> content </textArea>",
                parsed => Html::new(vec!["<pre> content </textArea>"])
            );
            success_case!(should_work_with_singe_line_abruptly_ending,
                "<script",
                parsed => Html::new(vec!["<script"])
            );
            success_case!(should_work_with_multi_line, "<PRe>\nline 2\n</sCRipt>\n", parsed => {
                Html::new(vec!["<PRe>\n", "line 2\n", "</sCRipt>\n"])
            });
            success_case!(should_work_with_multi_line_abruptly_ending,
                "<STYLE>\nline 2\nline 3\n",
                parsed => {
                    Html::new(vec!["<STYLE>\n", "line 2\n", "line 3\n"])
                }
            );
        }

        mod case_2 {
            use super::*;

            success_case!(
                should_work_with_single_line,
                "<!-- comment --> toto",
                parsed => Html::new(vec!["<!-- comment --> toto"])
            );
            success_case!(should_work_with_singe_line_abruptly_ending,
                "<!--",
                parsed => Html::new(vec!["<!--"])
            );
            success_case!(should_work_with_multi_line, "<!-- comment\nline 2 -->\n", parsed => {
                Html::new(vec!["<!-- comment\n", "line 2 -->\n"])
            });
            success_case!(should_work_with_multi_line_abruptly_ending,
                "<!-- comment\nline 2\nline 3\n",
                parsed => {
                    Html::new(vec!["<!-- comment\n", "line 2\n", "line 3\n"])
                }
            );
        }

        mod case_3 {
            use super::*;

            success_case!(
                should_work_with_single_line,
                "<? processing instruction ?> toto",
                parsed => Html::new(vec!["<? processing instruction ?> toto"])
            );
            success_case!(should_work_with_singe_line_abruptly_ending,
                "<?",
                parsed => Html::new(vec!["<?"])
            );
            success_case!(should_work_with_multi_line, "<? processing instruction\nline 2 ?>\n", parsed => {
                Html::new(vec!["<? processing instruction\n", "line 2 ?>\n"])
            });
            success_case!(should_work_with_multi_line_abruptly_ending,
                "<? processing instruction\nline 2\nline 3\n",
                parsed => {
                    Html::new(vec!["<? processing instruction\n", "line 2\n", "line 3\n"])
                }
            );
        }

        mod case_4 {
            use super::*;

            success_case!(
                should_work_with_single_line,
                "<!x> toto",
                parsed => Html::new(vec!["<!x> toto"])
            );
            success_case!(should_work_with_singe_line_abruptly_ending,
                "<!t",
                parsed => Html::new(vec!["<!t"])
            );
            success_case!(should_work_with_multi_line, "<!DOCTYPE html\nline 2>\n", parsed => {
                Html::new(vec!["<!DOCTYPE html\n", "line 2>\n"])
            });
            success_case!(should_work_with_multi_line_abruptly_ending,
                "<!DOCTYPE html\nline 2\nline 3\n",
                parsed => {
                    Html::new(vec!["<!DOCTYPE html\n", "line 2\n", "line 3\n"])
                }
            );
        }

        mod case_5 {
            use super::*;

            success_case!(
                should_work_with_single_line,
                "<![CDATA[]]> toto",
                parsed => Html::new(vec!["<![CDATA[]]> toto"])
            );
            success_case!(should_work_with_singe_line_abruptly_ending,
                "<![CDATA[",
                parsed => Html::new(vec!["<![CDATA["])
            );
            success_case!(should_work_with_multi_line, "<![CDATA[ content\nline 2 ]]> \n", parsed => {
                Html::new(vec!["<![CDATA[ content\n", "line 2 ]]> \n"])
            });
            success_case!(should_work_with_multi_line_abruptly_ending,
                "<![CDATA[ content\nline 2\nline 3\n",
                parsed => {
                    Html::new(vec!["<![CDATA[ content\n", "line 2\n", "line 3\n"])
                }
            );
        }

        mod case_6 {
            use super::*;

            success_case!(
                should_work_with_single_line,
                "<div> content </DIV>",
                parsed => Html::new(vec!["<div> content </DIV>"])
            );
            success_case!(should_work_with_singe_line_abruptly_ending,
                "<div",
                parsed => Html::new(vec!["<div"])
            );
            success_case!(should_stop_before_blank_line, "<dIV>\nline 2\n</DiV>\nHello\n\n", parsed => {
                Html::new(vec!["<dIV>\n", "line 2\n", "</DiV>\n", "Hello\n"])
            }, "\n");
        }

        mod case_7 {
            use super::*;

            success_case!(
                should_work_with_single_line,
                "<Big-Warning>\nI'm about to POP!</Big-Warning>",
                parsed => Html::new(vec!["<Big-Warning>\n", "I'm about to POP!</Big-Warning>"])
            );
            success_case!(
                should_work_with_complete_open_tag,
                "<a href=\"foo\">\n*bar*\n</a>",
                parsed => { Html::new(vec!["<a href=\"foo\">\n", "*bar*\n", "</a>"]) }
            );
        }
    }
}
