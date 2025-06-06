use crate::parse::blocks::block::macros::html_case_macro;

// This is case 3 in the spec, and covers lines with the following
// start and end conditions:
// - Start condition: line begins with the string <?.
// - End condition: line contains the string ?>.
html_case_macro!(HtmlCase3);

pub mod open {
    use super::*;
    use crate::parse::blocks::{
        block::macros::open::html_case_macro,
        open_block::{IOpenBlock, SingleSegmentBlock, Staging},
    };

    html_case_macro!(HtmlCase3, super);

    mod multi {
        use super::*;

        #[derive(Debug, Clone, PartialEq, Eq)]
        pub struct MultiLine<'a> {
            opening: parse::Opening<'a>,
            content: Vec<&'a str>,
            closing: Option<parse::Closing<'a>>,
            staging: Staging<parse::ContentOrClosing<'a>>,
        }

        impl<'a> MultiLine<'a> {
            pub fn new(opening: parse::Opening<'a>) -> Self {
                Self {
                    opening,
                    content: Vec::new(),
                    closing: None,
                    staging: Staging::new(),
                }
            }
        }

        impl<'a> IOpenBlock<'a> for MultiLine<'a> {
            type Closed = super::MultiLine<'a>;

            fn stage(&mut self, line: &'a str) -> Result<&'a str, ()> {
                if self.closing.is_some() {
                    return Err(());
                }

                let (remaining, content_or_closing) =
                    parse::content_or_closing(line).map_err(|_| ())?;
                self.staging.set(content_or_closing);
                Ok(remaining)
            }

            fn commit(&mut self) {
                self.staging
                    .commit(|content_or_closing| match content_or_closing {
                        parse::ContentOrClosing::Content(content) => {
                            self.content.push(content);
                        }
                        parse::ContentOrClosing::Closing(closing) => {
                            self.closing = Some(closing);
                        }
                    });
            }

            fn close<F: FnMut(Self::Closed) -> ()>(self, mut sink: F) {
                sink(super::MultiLine::new(
                    self.opening,
                    self.content,
                    self.closing,
                ));
            }
        }

        #[cfg(test)]
        mod test {
            use super::parse;
            use crate::parse::blocks::block::leaf::html::case_3 as closed;
            use crate::parse::blocks::open_block::IBlockTestExt;
            use crate::parse::blocks::open_block::IOpenBlock;
            use crate::parse::blocks::open_block::IOpenBlockTestExt;
            use parser::StrictParse;

            type Block<'a> = closed::HtmlCase3<'a>;

            #[test]
            fn should_reject_staging_after_closing() {
                let mut block = Block::open_and_commit_all("<? some long ass\n comment ?>\n");
                assert!(block.stage("content?\n").is_err());
            }

            #[test]
            fn should_work_when_closed_right_after_opening() {
                let block = Block::open_and_close("<?\n");
                assert_eq!(
                    Block::MultiLine(closed::MultiLine::new(
                        parse::opening.strict_parse("<?\n"),
                        vec![],
                        None
                    )),
                    block
                )
            }

            #[test]
            fn should_discard_staged_content_if_closed_before_committing() {
                let mut block = Block::strict_open("<?\n");
                block.stage("let x = 42;\n").unwrap();
                assert_eq!(
                    Block::MultiLine(closed::MultiLine::new(
                        parse::opening.strict_parse("<?\n"),
                        vec![],
                        None
                    )),
                    block.close_and_return()
                )
            }

            #[test]
            fn should_work_when_closed_before_reaching_closing() {
                let block = Block::open_and_close("<?\nAin't no closure here.");
                assert_eq!(
                    Block::MultiLine(closed::MultiLine::new(
                        parse::opening.strict_parse("<?\n"),
                        vec!["Ain't no closure here."],
                        None
                    )),
                    block
                )
            }

            #[test]
            fn should_work_when_closed_after_reaching_closing() {
                let block = Block::open_and_close("<?\n closed ?>\n");
                assert_eq!(
                    Block::MultiLine(closed::MultiLine::new(
                        parse::opening.strict_parse("<?\n"),
                        vec![],
                        Some(parse::closing.strict_parse(" closed ?>\n"))
                    )),
                    block
                );
            }

            #[test]
            fn should_work_when_closed_with_content_and_closing() {
                let block = Block::open_and_close("<?\nlet x = 42;\n?>\n");
                assert_eq!(
                    Block::MultiLine(closed::MultiLine::new(
                        parse::opening.strict_parse("<?\n"),
                        vec!["let x = 42;\n"],
                        Some(parse::closing.strict_parse("?>\n"))
                    )),
                    block
                );
            }
        }
    }

    mod single {
        use super::*;
        #[derive(Debug, Clone, PartialEq, Eq)]
        pub struct SingleLine<'a>(parse::OpeningAndClosing<'a>);

        impl<'a> SingleLine<'a> {
            pub fn new(opening: parse::OpeningAndClosing<'a>) -> Self {
                Self(opening)
            }
        }

        impl<'a> From<SingleLine<'a>> for super::SingleLine<'a> {
            fn from(single_line: SingleLine<'a>) -> Self {
                super::SingleLine::new(single_line.0)
            }
        }

        impl<'a> SingleSegmentBlock<'a> for SingleLine<'a> {
            type Closed = super::SingleLine<'a>;
        }

        #[cfg(test)]
        mod test {
            use super::parse;
            use crate::parse::blocks::block::leaf::html::case_3 as closed;
            use crate::parse::blocks::open_block::IBlockTestExt;
            use crate::parse::blocks::open_block::IOpenBlock;
            use parser::StrictParse;

            type Block<'a> = closed::HtmlCase3<'a>;

            #[test]
            fn should_should_reject_staging_after_closing() {
                let mut block = Block::open_and_commit_all("<? toto ?>\n");
                assert!(block.stage("content?\n").is_err());
            }

            #[test]
            fn should_produce_the_expected_result() {
                let block = Block::open_and_close("<? toto ?> tata?\n");
                assert_eq!(
                    Block::SingleLine(closed::SingleLine::new(
                        parse::opening_maybe_closing
                            .strict_parse("<? toto ?> tata?\n")
                            .unwrap_opening_and_closing()
                    )),
                    block
                );
            }
        }
    }
}

mod parse {
    pub use closing::*;
    pub use content_or_closing::*;
    pub use opening::*;
    pub use opening_maybe_closing::*;

    mod opening_maybe_closing {
        use super::*;
        use parser::ParseResult;

        pub fn opening_maybe_closing(line: &str) -> ParseResult<&str, OpeningMaybeClosing> {
            let (remaining, opening) = opening(line)?;
            match closing(line) {
                Ok((remaining, _)) => Ok((remaining, OpeningAndClosing::new(line).into())),
                Err(_) => Ok((remaining, opening.into())),
            }
        }

        #[derive(Debug, Clone, PartialEq, Eq)]
        pub enum OpeningMaybeClosing<'a> {
            Opening(Opening<'a>),
            OpeningAndClosing(OpeningAndClosing<'a>),
        }

        impl<'a> OpeningMaybeClosing<'a> {
            #[cfg(test)]
            pub fn unwrap_opening_and_closing(self) -> OpeningAndClosing<'a> {
                match self {
                    OpeningMaybeClosing::Opening(_) => {
                        panic!("cannot unwrap opening and closing from: {:?}", self)
                    }
                    OpeningMaybeClosing::OpeningAndClosing(opening_and_closing) => {
                        opening_and_closing
                    }
                }
            }
        }

        impl<'a> From<Opening<'a>> for OpeningMaybeClosing<'a> {
            fn from(opening: Opening<'a>) -> Self {
                OpeningMaybeClosing::Opening(opening)
            }
        }

        impl<'a> From<OpeningAndClosing<'a>> for OpeningMaybeClosing<'a> {
            fn from(opening_and_closing: OpeningAndClosing<'a>) -> Self {
                OpeningMaybeClosing::OpeningAndClosing(opening_and_closing)
            }
        }

        #[derive(Debug, Clone, PartialEq, Eq)]
        pub struct OpeningAndClosing<'a>(&'a str);

        impl<'a> OpeningAndClosing<'a> {
            fn new(opening: &'a str) -> Self {
                Self(opening)
            }
        }
    }

    mod opening {
        use crate::parse::blocks::block::leaf::html::macros::parse::opening_macro;

        opening_macro!(|line: &str| { line.starts_with("<?") });

        #[cfg(test)]
        mod test {
            use super::*;

            #[test]
            fn should_fail_with_empty() {
                assert_eq!(Err(""), opening(""));
            }

            #[test]
            fn should_fail_with_blank_line() {
                assert_eq!(Err("\n"), opening("\n"));
            }

            #[test]
            fn should_fail_with_text_before_tag() {
                assert_eq!(Err("some text before <?"), opening("some text before <?"));
            }

            #[test]
            fn should_fail_with_invalid_tag() {
                assert_eq!(Err("<! nope !>"), opening("<! nope !>"));
            }

            #[test]
            fn should_work_for_valid_tag() {
                assert_eq!(Ok(("", Opening::new("<?"))), opening("<?"),);
            }

            #[test]
            fn should_work_with_newline_after_tag() {
                assert_eq!(Ok(("", Opening::new("<?\n"))), opening("<?\n"),);
            }

            #[test]
            fn should_work_with_any_text_after() {
                assert_eq!(
                    Ok(("", Opening::new("<?some text after"))),
                    opening("<?some text after"),
                );
            }
        }
    }

    mod closing {
        use crate::parse::blocks::block::macros::parse::closing_macro;

        closing_macro!(|line: &str| { line.contains("?>") });

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
                assert_eq!(Err("<?"), closing("<?"));
            }

            #[test]
            fn should_work_for_valid_tag() {
                assert_eq!(Ok(("", Closing::new("?>"))), closing("?>"),);
            }

            #[test]
            fn should_work_with_text_before_tag() {
                assert_eq!(
                    Ok(("", Closing::new("some text before ?>"))),
                    closing("some text before ?>"),
                );
            }

            #[test]
            fn should_work_with_text_after_tag() {
                assert_eq!(
                    Ok(("", Closing::new("?> some text after\n"))),
                    closing("?> some text after\n"),
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
                    Ok(("", ContentOrClosing::Closing(Closing::new("?>\n")))),
                    content_or_closing("?>\n"),
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
