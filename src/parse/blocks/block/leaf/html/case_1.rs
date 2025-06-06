use crate::parse::blocks::block::macros::html_case_macro;

pub const CASE_1_TAG_NAMES: [&str; 4] = ["pre", "script", "style", "textarea"];

// This is case 1 in the spec, and covers lines with the following
// - Start condition: line begins with the string <pre, <script, <style, or <textarea (case-insensitive),
//   followed by a space, a tab, the string >, or the end of the line.
// - End condition: line contains an end tag </pre>, </script>, </style>, or </textarea> (case-insensitive; it need not match the start tag).
html_case_macro!(HtmlCase1);

pub mod open {
    use crate::parse::blocks::block::macros::open::html_case_macro;

    html_case_macro!(crate::parse::blocks::block::leaf::html::case_1, HtmlCase1);

    mod multi {
        use crate::parse::blocks::block::macros::open::multiline_macro;

        multiline_macro!(crate::parse::blocks::block::leaf::html::case_1);

        #[cfg(test)]
        mod test {
            use crate::parse::blocks::block::leaf::html::case_1 as closed;
            use crate::parse::blocks::block::leaf::html::case_1::parse;
            use crate::parse::blocks::open_block::IBlockTestExt;
            use crate::parse::blocks::open_block::IOpenBlock;
            use crate::parse::blocks::open_block::IOpenBlockTestExt;
            use parser::StrictParse;

            type Block<'a> = closed::HtmlCase1<'a>;

            #[test]
            fn should_reject_staging_after_closing() {
                let mut block = Block::open_and_commit_all("<pre>\n</script>\n");
                assert!(block.stage("content?\n").is_err());
            }

            #[test]
            fn should_work_when_closed_right_after_opening() {
                let block = Block::open_and_close("<textarea>\n");
                assert_eq!(
                    Block::MultiLine(closed::MultiLine::new(
                        parse::opening.strict_parse("<textarea>\n"),
                        vec![],
                        None
                    )),
                    block
                )
            }

            #[test]
            fn should_discard_staged_content_if_closed_before_committing() {
                let mut block = Block::strict_open("<script>\n");
                block.stage("let x = 42;\n").unwrap();
                assert_eq!(
                    Block::MultiLine(closed::MultiLine::new(
                        parse::opening.strict_parse("<script>\n"),
                        vec![],
                        None
                    )),
                    block.close_and_return()
                )
            }

            #[test]
            fn should_work_when_closed_before_reaching_closing() {
                let block = Block::open_and_close("<style>\nAin't no style here.");
                assert_eq!(
                    Block::MultiLine(closed::MultiLine::new(
                        parse::opening.strict_parse("<style>\n"),
                        vec!["Ain't no style here."],
                        None
                    )),
                    block
                )
            }

            #[test]
            fn should_work_when_closed_after_reaching_closing() {
                let block = Block::open_and_close("<style>\n</textarea>\n");
                assert_eq!(
                    Block::MultiLine(closed::MultiLine::new(
                        parse::opening.strict_parse("<style>\n"),
                        vec![],
                        Some(parse::closing.strict_parse("</textarea>\n"))
                    )),
                    block
                );
            }

            #[test]
            fn should_work_when_closed_with_content_and_closing() {
                let block = Block::open_and_close("<style>\nlet x = 42;\n</script>\n");
                assert_eq!(
                    Block::MultiLine(closed::MultiLine::new(
                        parse::opening.strict_parse("<style>\n"),
                        vec!["let x = 42;\n"],
                        Some(parse::closing.strict_parse("</script>\n"))
                    )),
                    block
                );
            }
        }
    }

    mod single {
        use crate::parse::blocks::block::macros::open::single_line_macro;

        single_line_macro!();

        #[cfg(test)]
        mod test {
            use crate::parse::blocks::block::leaf::html::case_1 as closed;
            use crate::parse::blocks::open_block::IBlockTestExt;
            use crate::parse::blocks::open_block::IOpenBlock;
            use closed::parse;
            use parser::StrictParse;

            type Block<'a> = closed::HtmlCase1<'a>;

            #[test]
            fn should_should_reject_staging_after_closing() {
                let mut block = Block::open_and_commit_all("<pre>toto</pre>\n");
                assert!(block.stage("content?\n").is_err());
            }

            #[test]
            fn should_produce_the_expected_result() {
                let block = Block::open_and_close("<pre>toto</pre>\n");
                assert_eq!(
                    Block::SingleLine(closed::SingleLine::new(
                        parse::opening_maybe_closing
                            .strict_parse("<pre>toto</pre>\n")
                            .unwrap_opening_and_closing()
                    )),
                    block
                );
            }
        }
    }
}

mod parse {
    use crate::parse::blocks::block::macros::parse::{
        closing_module_macro, content_or_closing_module_macro, opening_maybe_closing_module_macro,
        opening_module_macro,
    };
    pub use closing::*;
    pub use content_or_closing::*;
    pub use opening::*;
    pub use opening_maybe_closing::*;

    opening_module_macro!(
        |line: &str| {
            use crate::parse::blocks::block::leaf::html::case_1::CASE_1_TAG_NAMES;
            use crate::parse::blocks::block::leaf::html::parsers::tag_name;
            use crate::parse::parsers::indented_by_less_than_4;
            use parser::{Parser, empty, is_one_of, one_of, recognize, tag, take, validate};

            recognize((
                indented_by_less_than_4,
                tag("<"),
                validate(tag_name, |tag_name: &&str| {
                    CASE_1_TAG_NAMES
                        .iter()
                        .any(|name| name.eq_ignore_ascii_case(tag_name))
                }),
                one_of((take(1).that(is_one_of(&[' ', '\t', '\n', '>'])), empty)),
            ))
            .parse(line)
            .is_ok()
        },
        [
            "<pre ",
            "<PRE ",
            "<pre \t",
            "<pre>",
            "<script ",
            "<SCRIPT ",
            "<script \t",
            "<script>",
            "<style ",
            "<STYLE ",
            "<style \t",
            "<style>",
            "<textarea ",
            "<TEXTAREA ",
            "<textarea \t",
            "<textarea>"
        ],
        "<javacunt>"
    );
    closing_module_macro!(
        |line: &str| {
            // TODO: implement this without allocating.
            let lowercased = line.to_lowercase();
            lowercased.contains("</pre>")
                || lowercased.contains("</script>")
                || lowercased.contains("</style>")
                || lowercased.contains("</textarea>")
        },
        [
            "</pre>",
            "</PRE>",
            "</script>",
            "</SCRIPT>",
            "</style>",
            "</STYLE>",
            "</textarea>",
            "</TEXTAREA>",
        ],
        "<javapiss>"
    );
    opening_maybe_closing_module_macro!("<pre>", "<pre></pre>");
    content_or_closing_module_macro!("word\n", "</pre>\n");
}
