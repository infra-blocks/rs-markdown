use super::parsers;
use crate::parse::blocks::block::macros::html_case_macro;

// This is case 1 in the spec, and covers lines with the following
// - Start condition: line begins with the string <pre, <script, <style, or <textarea (case-insensitive),
//   followed by a space, a tab, the string >, or the end of the line.
// - End condition: line contains an end tag </pre>, </script>, </style>, or </textarea> (case-insensitive; it need not match the start tag).
html_case_macro!(HtmlCase1);

pub mod open {
    use crate::parse::blocks::block::macros::open::html_case_macro;

    html_case_macro!(HtmlCase1, super);

    mod multi {
        use crate::parse::blocks::block::macros::open::multiline_macro;

        multiline_macro!(super::super);

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

        single_line_macro!(super::super);

        #[cfg(test)]
        mod test {
            use crate::parse::blocks::block::leaf::html::case_1 as closed;
            use crate::parse::blocks::block::leaf::html::case_1::parse;
            use crate::parse::blocks::open_block::IBlockTestExt;
            use crate::parse::blocks::open_block::IOpenBlock;
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
    use super::parsers;
    pub use closing::*;
    pub use content_or_closing::*;
    pub use opening::*;
    pub use opening_maybe_closing::*;

    const CASE_1_TAG_NAMES: [&str; 4] = ["pre", "script", "style", "textarea"];

    mod opening_maybe_closing {
        use super::*;
        use crate::parse::blocks::block::macros::parse::opening_maybe_closing_macro;

        opening_maybe_closing_macro!(Opening<'a>, opening, closing);

        #[cfg(test)]
        mod test {
            use super::*;
            use parser::StrictParse;

            #[test]
            fn should_work_for_opening() {
                assert_eq!(
                    Ok((
                        "",
                        OpeningMaybeClosing::Opening(opening.strict_parse("<pre>"))
                    )),
                    opening_maybe_closing("<pre>"),
                );
            }

            #[test]
            fn should_work_for_opening_and_closing() {
                assert_eq!(
                    Ok((
                        "",
                        OpeningMaybeClosing::OpeningAndClosing(OpeningAndClosing::new(
                            "<pre></pre>"
                        ))
                    )),
                    opening_maybe_closing("<pre></pre>"),
                );
            }
        }
    }

    mod opening {
        use super::*;
        use crate::parse::{
            blocks::block::macros::parse::opening_macro, parsers::indented_by_less_than_4,
        };
        use parser::{Parser, empty, is_one_of, one_of, recognize, tag, take, validate};

        opening_macro!(|line: &str| {
            recognize((
                indented_by_less_than_4,
                tag("<"),
                validate(parsers::tag_name, |tag_name: &&str| {
                    CASE_1_TAG_NAMES
                        .iter()
                        .any(|name| name.eq_ignore_ascii_case(tag_name))
                }),
                one_of((take(1).that(is_one_of(&[' ', '\t', '\n', '>'])), empty)),
            ))
            .parse(line)
            .is_ok()
        });

        #[cfg(test)]
        mod test {
            use super::*;

            #[test]
            fn should_fail_with_text_before_tag() {
                assert_eq!(
                    Err("some text before <pre>"),
                    opening("some text before <pre>")
                );
            }

            #[test]
            fn should_fail_with_invalid_tag() {
                assert_eq!(Err("<javascript>"), opening("<javascript>"));
            }

            #[test]
            fn should_work_for_script() {
                assert_eq!(Ok(("", Opening::new("<script>"))), opening("<script>"),);
            }

            #[test]
            fn should_work_for_pre() {
                assert_eq!(Ok(("", Opening::new("<pre>"))), opening("<pre>"),);
            }

            #[test]
            fn should_work_for_style() {
                assert_eq!(Ok(("", Opening::new("<style>\n"))), opening("<style>\n"),);
            }

            #[test]
            fn should_work_for_textarea() {
                assert_eq!(
                    Ok(("", Opening::new("<textarea>\n"))),
                    opening("<textarea>\n"),
                );
            }

            #[test]
            fn should_work_with_missing_closing_bracket() {
                assert_eq!(Ok(("", Opening::new("<pre"))), opening("<pre"),);
            }

            #[test]
            fn should_work_with_space_after_tag_name() {
                assert_eq!(Ok(("", Opening::new("<pre "))), opening("<pre "),);
            }

            #[test]
            fn should_work_with_tab_after_tag_name() {
                assert_eq!(Ok(("", Opening::new("<pre\t"))), opening("<pre\t"),);
            }

            #[test]
            fn should_work_with_newline_after_tag_name() {
                assert_eq!(Ok(("", Opening::new("<pre\n"))), opening("<pre\n"),);
            }
        }
    }

    mod closing {
        use crate::parse::blocks::block::macros::parse::closing_macro;

        closing_macro!(|line: &str| {
            // TODO: implement this without allocating.
            let lowercased = line.to_lowercase();
            lowercased.contains("</pre>")
                || lowercased.contains("</script>")
                || lowercased.contains("</style>")
                || lowercased.contains("</textarea>")
        });

        #[cfg(test)]
        mod test {
            use super::*;

            #[test]
            fn should_fail_with_empty() {
                assert_eq!(Err(""), closing(""));
            }

            #[test]
            fn should_fail_with_blank_line() {
                assert_eq!(Err("\n"), closing("\n"));
            }

            #[test]
            fn should_fail_open_tag() {
                assert_eq!(Err("<pre>"), closing("<pre>"));
            }

            #[test]
            fn should_work_for_script() {
                assert_eq!(
                    Ok(("", Closing::new("</script>\n"))),
                    closing("</script>\n"),
                );
            }

            #[test]
            fn should_work_pre() {
                assert_eq!(Ok(("", Closing::new("</pre>\n"))), closing("</pre>\n"),);
            }

            #[test]
            fn should_work_with_style() {
                assert_eq!(Ok(("", Closing::new("</style>"))), closing("</style>"),);
            }

            #[test]
            fn should_work_for_textarea() {
                assert_eq!(
                    Ok(("", Closing::new("</textarea>\n"))),
                    closing("</textarea>\n"),
                );
            }

            #[test]
            fn should_work_with_text_before_tag() {
                assert_eq!(
                    Ok(("", Closing::new("some text before </pre>\n"))),
                    closing("some text before </pre>\n"),
                );
            }

            #[test]
            fn should_work_with_text_after_tag() {
                assert_eq!(
                    Ok(("", Closing::new("</pre> some text after\n"))),
                    closing("</pre> some text after\n"),
                );
            }
        }
    }

    mod content_or_closing {
        use super::*;
        use crate::parse::blocks::block::macros::parse::content_or_closing_macro;

        content_or_closing_macro!(Closing<'a>);

        #[cfg(test)]
        mod test {
            use super::*;

            #[test]
            fn should_work_for_closing() {
                assert_eq!(
                    Ok(("", ContentOrClosing::Closing(Closing::new("</pre>\n")))),
                    content_or_closing("</pre>\n"),
                );
            }

            #[test]
            fn should_default_to_content_when_not_closing() {
                assert_eq!(
                    Ok(("", ContentOrClosing::Content("word\n"))),
                    content_or_closing("word\n"),
                );
            }
        }
    }
}
