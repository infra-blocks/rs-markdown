use crate::parse::blocks::block::macros::{html_case_macro, open_module_macro};

pub const CASE_6_TAG_NAMES: [&str; 62] = [
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

// This is case 6 in the spec, and covers lines with the following
// start and end conditions:
// - Start condition: line begins with the string < or </ followed by one of the strings (case-insensitive) address,
//   article, aside, base, basefont, blockquote, body, caption, center, col, colgroup, dd, details, dialog, dir,
//   div, dl, dt, fieldset, figcaption, figure, footer, form, frame, frameset, h1, h2, h3, h4, h5, h6, head, header,
//   hr, html, iframe, legend, li, link, main, menu, menuitem, nav, noframes, ol, optgroup, option, p, param, search,
//   section, summary, table, tbody, td, tfoot, th, thead, title, tr, track, ul, followed by a space, a tab,
//   the end of the line, the string >, or the string />.
// - End condition: line is followed by a blank line.
html_case_macro!(HtmlCase6);

mod parse {
    use crate::parse::blocks::block::macros::parse::{
        closing_module_macro, content_or_closing_module_macro, opening_maybe_closing_module_macro,
    };
    pub use closing::*;
    pub use content_or_closing::*;
    pub use opening::*;
    pub use opening_maybe_closing::*;

    mod opening {
        use crate::parse::blocks::block::macros::parse::opening_macro;

        opening_macro!(|line: &str| {
            use crate::parse::blocks::block::leaf::html::case_6::CASE_6_TAG_NAMES;
            use crate::parse::blocks::block::leaf::html::parsers::tag_name;
            use crate::parse::parsers::indented_by_less_than_4;
            use parser::{Parser, empty, is_one_of, one_of, tag, take, validate};

            (
                indented_by_less_than_4,
                (
                    one_of((tag("<"), tag("</"))),
                    validate(tag_name, |tag_name: &&str| {
                        CASE_6_TAG_NAMES
                            .iter()
                            .any(|name| name.eq_ignore_ascii_case(tag_name))
                    }),
                    one_of((
                        take(1).that(is_one_of(&[' ', '\t', '\n', '>'])),
                        tag("/>"),
                        empty,
                    )),
                ),
            )
                .parse(line)
                .is_ok()
        });
    }

    closing_module_macro!(|line: &str| line.contains("]]>"), "]]>", "]>");
    opening_maybe_closing_module_macro!("<![CDATA[\n", "<![CDATA[ cunt ]]>\n");
    content_or_closing_module_macro!("word\n", "]]>\n");
}
