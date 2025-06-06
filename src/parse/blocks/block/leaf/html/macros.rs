macro_rules! html_case_macro {
    ($case:ident) => {
        #[derive(Debug, Clone, PartialEq, Eq)]
        pub enum $case<'a> {
            SingleLine(SingleLine<'a>),
            MultiLine(MultiLine<'a>),
        }

        impl<'a> From<SingleLine<'a>> for $case<'a> {
            fn from(single_line: SingleLine<'a>) -> Self {
                $case::SingleLine(single_line)
            }
        }

        impl<'a> From<MultiLine<'a>> for $case<'a> {
            fn from(multiline: MultiLine<'a>) -> Self {
                $case::MultiLine(multiline)
            }
        }

        #[derive(Debug, Clone, PartialEq, Eq)]
        pub struct SingleLine<'a>(parse::OpeningAndClosing<'a>);

        impl<'a> SingleLine<'a> {
            pub fn new(opening: parse::OpeningAndClosing<'a>) -> Self {
                Self(opening)
            }
        }

        #[derive(Debug, Clone, PartialEq, Eq)]
        pub struct MultiLine<'a> {
            opening: parse::Opening<'a>,
            content: Vec<&'a str>,
            closing: Option<parse::Closing<'a>>,
        }

        impl<'a> MultiLine<'a> {
            fn new(
                opening: parse::Opening<'a>,
                content: Vec<&'a str>,
                closing: Option<parse::Closing<'a>>,
            ) -> Self {
                Self {
                    opening,
                    content,
                    closing,
                }
            }
        }

        impl<'a> crate::parse::blocks::open_block::IBlock<'a> for $case<'a> {
            type Open = open::$case<'a>;

            fn open(line: &'a str) -> parser::ParseResult<&'a str, Self::Open> {
                use parser::{Map, Parser};

                parse::opening_maybe_closing
                    .map(open::$case::new)
                    .parse(line)
            }
        }
    };
}
pub(super) use html_case_macro;

macro_rules! open_module_macro {
    ($($root:ident)::+, $case:ident, $opening:expr, $closing:expr) => {
        pub mod open {
            use crate::parse::blocks::block::macros::open::{
                html_case_macro, multi_line_module_macro, single_line_module_macro,
            };

            html_case_macro!($($root)::+, $case);
            multi_line_module_macro!($($root)::+, $case, $opening, $closing);
            single_line_module_macro!($($root)::+, $case, $opening, $closing);
        }
    };
}
pub(in super::super) use open_module_macro;

pub mod open {
    macro_rules! html_case_macro {
        ($($root:ident)::+, $case:ident) => {
            #[derive(Debug, Clone, PartialEq, Eq)]
            pub enum $case<'a> {
                SingleLine(single::SingleLine<'a>),
                MultiLine(multi::MultiLine<'a>),
            }

            impl<'a> $case<'a> {
                pub fn new(opening: $($root)::+::parse::OpeningMaybeClosing<'a>) -> Self {
                    match opening {
                        $($root)::+::parse::OpeningMaybeClosing::Opening(opening) => {
                            Self::MultiLine(multi::MultiLine::new(opening))
                        }
                        $($root)::+::parse::OpeningMaybeClosing::OpeningAndClosing(opening_and_closing) => {
                            Self::SingleLine(single::SingleLine::new(opening_and_closing))
                        }
                    }
                }
            }

            impl<'a> crate::parse::blocks::open_block::IOpenBlock<'a> for $case<'a> {
                type Closed = $($root)::+::$case<'a>;

                fn stage(&mut self, line: &'a str) -> Result<&'a str, ()> {
                    match self {
                        Self::SingleLine(block) => block.stage(line),
                        Self::MultiLine(block) => block.stage(line),
                    }
                }

                fn commit(&mut self) {
                    match self {
                        Self::SingleLine(block) => block.commit(),
                        Self::MultiLine(block) => block.commit(),
                    }
                }

                fn close<F: FnMut(Self::Closed) -> ()>(self, mut sink: F) {
                    match self {
                        Self::SingleLine(block) => {
                            block.close(|single_line| sink(single_line.into()))
                        }
                        Self::MultiLine(block) => {
                            block.close(|multi_line| sink(multi_line.into()))
                        }
                    }
                }
            }
        };
    }
    pub(in super::super) use html_case_macro;

    macro_rules! multiline_macro {
        ($($root:ident)::+) => {
            #[derive(Debug, Clone, PartialEq, Eq)]
            pub struct MultiLine<'a> {
                opening: $($root)::+::parse::Opening<'a>,
                content: Vec<&'a str>,
                closing: Option<$($root)::+::parse::Closing<'a>>,
                staging: crate::parse::blocks::open_block::Staging<$($root)::+::parse::ContentOrClosing<'a>>,
            }

            impl<'a> MultiLine<'a> {
                pub fn new(opening: $($root)::+::parse::Opening<'a>) -> Self {
                    Self {
                        opening,
                        content: Vec::new(),
                        closing: None,
                        staging: crate::parse::blocks::open_block::Staging::new(),
                    }
                }
            }

            impl<'a> crate::parse::blocks::open_block::IOpenBlock<'a> for MultiLine<'a> {
                type Closed = $($root)::+::MultiLine<'a>;

                fn stage(&mut self, line: &'a str) -> Result<&'a str, ()> {
                    if self.closing.is_some() {
                        return Err(());
                    }

                    let (remaining, content_or_closing) =
                        $($root)::+::parse::content_or_closing(line).map_err(|_| ())?;
                    self.staging.set(content_or_closing);
                    Ok(remaining)
                }

                fn commit(&mut self) {
                    self.staging
                        .commit(|content_or_closing| match content_or_closing {
                            $($root)::+::parse::ContentOrClosing::Content(content) => {
                                self.content.push(content);
                            }
                            $($root)::+::parse::ContentOrClosing::Closing(closing) => {
                                self.closing = Some(closing);
                            }
                        });
                }

                fn close<F: FnMut(Self::Closed) -> ()>(self, mut sink: F) {
                    sink($($root)::+::MultiLine::new(
                        self.opening,
                        self.content,
                        self.closing,
                    ));
                }
            }
        };
    }
    pub(in super::super) use multiline_macro;

    macro_rules! multi_line_tests_macro {
        ($($root:ident)::+, $case:ident, $opening:expr, $closing:expr) => {
            #[cfg(test)]
            mod test {
                use $($root)::+::parse;
                use $($root)::+::$case;
                use crate::parse::blocks::open_block::IBlockTestExt;
                use crate::parse::blocks::open_block::IOpenBlock;
                use crate::parse::blocks::open_block::IOpenBlockTestExt;
                use parser::StrictParse;

                #[test]
                fn should_reject_staging_after_closing() {
                    let text = concat!($opening, " some long ass\n comment ", $closing, "\n");
                    let mut block = $case::open_and_commit_all(text);
                    assert!(block.stage("nope?\n").is_err());
                }

                #[test]
                fn should_work_when_closed_right_after_opening() {
                    let block = $case::open_and_close($opening);
                    assert_eq!(
                        $case::MultiLine($($root)::+::MultiLine::new(
                            parse::opening.strict_parse($opening),
                            vec![],
                            None
                        )),
                        block
                    )
                }

                #[test]
                fn should_discard_staged_content_if_closed_before_committing() {
                    let first = concat!($opening, "\n");
                    let mut block = $case::strict_open(first);
                    block.stage("let x = 42;\n").unwrap();
                    assert_eq!(
                        $case::MultiLine($($root)::+::MultiLine::new(
                            parse::opening.strict_parse(first),
                            vec![],
                            None
                        )),
                        block.close_and_return()
                    )
                }

                #[test]
                fn should_work_when_closed_before_reaching_closing() {
                    let first = concat!($opening, "\n");
                    let second = "Ain't no closure here.";
                    let text = format!("{}{}", first, second);
                    let block = $case::open_and_close(text.as_str());
                    assert_eq!(
                        $case::MultiLine($($root)::+::MultiLine::new(
                            parse::opening.strict_parse(first),
                            vec![second],
                            None
                        )),
                        block
                    )
                }

                #[test]
                fn should_work_when_closed_after_reaching_closing() {
                    let first = concat!($opening, "\n");
                    let second = concat!("closed ", $closing, "\n");
                    let text = format!("{}{}", first, second);
                    let block = $case::open_and_close(text.as_str());
                    assert_eq!(
                        $case::MultiLine($($root)::+::MultiLine::new(
                            parse::opening.strict_parse(first),
                            vec![],
                            Some(parse::closing.strict_parse(second))
                        )),
                        block
                    );
                }

                #[test]
                fn should_work_when_closed_with_content_and_closing() {
                    let first = concat!($opening, "\n");
                    let second = "let x = 42;\n";
                    let third = concat!($closing, "\n");
                    let text = format!("{}{}{}", first, second, third);
                    let block = $case::open_and_close(text.as_str());
                    assert_eq!(
                        $case::MultiLine($($root)::+::MultiLine::new(
                            parse::opening.strict_parse(first),
                            vec![second],
                            Some(parse::closing.strict_parse(third))
                        )),
                        block
                    );
                }
            }
        };
    }
    pub(in super::super) use multi_line_tests_macro;

    macro_rules! multi_line_module_macro {
        ($($root:ident)::+, $case:ident, $opening:expr, $closing:expr) => {
            mod multi {
                use crate::parse::blocks::block::macros::open::{multi_line_tests_macro, multiline_macro};

                multiline_macro!($($root)::+);
                multi_line_tests_macro!($($root)::+, $case, $opening, $closing);
            }
        };
    }
    pub(in super::super) use multi_line_module_macro;

    macro_rules! single_line_macro {
        () => {
            single_line_macro!(super::super);
        };
        ($($root:ident)::+) => {
            #[derive(Debug, Clone, PartialEq, Eq)]
            pub struct SingleLine<'a>($($root)::+::parse::OpeningAndClosing<'a>);

            impl<'a> SingleLine<'a> {
                pub fn new(opening: $($root)::+::parse::OpeningAndClosing<'a>) -> Self {
                    Self(opening)
                }
            }

            impl<'a> From<SingleLine<'a>> for $($root)::+::SingleLine<'a> {
                fn from(single_line: SingleLine<'a>) -> Self {
                    $($root)::+::SingleLine::new(single_line.0)
                }
            }

            impl<'a> crate::parse::blocks::open_block::SingleSegmentBlock<'a> for SingleLine<'a> {
                type Closed = $($root)::+::SingleLine<'a>;
            }
        };
    }
    pub(in super::super) use single_line_macro;

    macro_rules! single_line_tests_macro {
        ($($root:ident)::+, $block:ident, $opening:literal, $closing:literal) => {
            #[cfg(test)]
            mod test {
                use $($root)::+::parse;
                use $($root)::+::$block;
                use crate::parse::blocks::open_block::IBlockTestExt;
                use crate::parse::blocks::open_block::IOpenBlock;
                use parser::StrictParse;

                #[test]
                fn should_should_reject_staging_after_closing() {
                    let text = concat!($opening, " toto ", $closing, "\n");
                    let mut block = $block::open_and_commit_all(text);
                    assert!(block.stage("content?\n").is_err());
                }

                #[test]
                fn should_produce_the_expected_result() {
                    let text = concat!($opening, " toto ", $closing, "tata\n");
                    let block = $block::open_and_close(text);
                    assert_eq!(
                        $block::SingleLine($($root)::+::SingleLine::new(
                            parse::opening_maybe_closing
                                .strict_parse(text)
                                .unwrap_opening_and_closing()
                        )),
                        block
                    );
                }
            }
        };
    }
    pub(in super::super) use single_line_tests_macro;

    macro_rules! single_line_module_macro {
        ($($root:ident)::+, $block:ident, $opening:literal, $closing:literal) => {
            mod single {
                use crate::parse::blocks::block::macros::open::{
                    single_line_macro, single_line_tests_macro,
                };

                single_line_macro!($($root)::+);
                single_line_tests_macro!($($root)::+, $block, $opening, $closing);
            }
        };
    }
    pub(in super::super) use single_line_module_macro;
}

pub mod parse {
    macro_rules! opening_macro {
        ($predicate:expr) => {
            pub fn opening(line: &str) -> parser::ParseResult<&str, Opening> {
                if $predicate(line) {
                    Ok(("", Opening::new(line)))
                } else {
                    Err(line)
                }
            }

            #[derive(Debug, Clone, PartialEq, Eq)]
            pub struct Opening<'a>(&'a str);

            impl<'a> Opening<'a> {
                fn new(opening: &'a str) -> Self {
                    Self(opening)
                }
            }
        };
    }
    pub(in super::super) use opening_macro;

    macro_rules! opening_tests_macro {
        ($valid_openings:expr, $invalid:expr) => {
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
                fn should_fail_with_text_before_opening() {
                    for valid in $valid_openings {
                        let text = format!("some text before {}", valid);
                        assert_eq!(Err(text.as_str()), opening(text.as_str()));
                    }
                }

                #[test]
                fn should_fail_with_invalid_opening() {
                    assert_eq!(Err($invalid), opening($invalid));
                }

                #[test]
                fn should_work_for_valid_opening() {
                    for valid in $valid_openings {
                        assert_eq!(Ok(("", Opening::new(valid))), opening(valid),);
                    }
                }

                #[test]
                fn should_work_with_3_spaces_indent() {
                    for valid in $valid_openings {
                        let text = format!("   {}", valid);
                        assert_eq!(
                            Ok(("", Opening::new(text.as_str()))),
                            opening(text.as_str())
                        );
                    }
                }

                #[test]
                fn should_work_with_newline_after_opening() {
                    for valid in $valid_openings {
                        let text = format!("{}\n", valid);
                        assert_eq!(
                            Ok(("", Opening::new(text.as_str()))),
                            opening(text.as_str())
                        );
                    }
                }

                #[test]
                fn should_work_with_any_text_after_opening() {
                    for valid in $valid_openings {
                        let text = format!("{}some text after", valid);
                        assert_eq!(
                            Ok(("", Opening::new(text.as_str()))),
                            opening(text.as_str())
                        );
                    }
                }
            }
        };
    }
    pub(in super::super) use opening_tests_macro;

    macro_rules! opening_module_macro {
        ($predicate:expr, $valid_openings:expr, $invalid:expr) => {
            mod opening {
                use crate::parse::blocks::block::macros::parse::{
                    opening_macro, opening_tests_macro,
                };

                opening_macro!($predicate);
                opening_tests_macro!($valid_openings, $invalid);
            }
        };
    }
    pub(in super::super) use opening_module_macro;

    macro_rules! closing_macro {
        ($predicate:expr) => {
            pub fn closing(line: &str) -> parser::ParseResult<&str, Closing> {
                if $predicate(line) {
                    Ok(("", Closing::new(line)))
                } else {
                    Err(line)
                }
            }

            #[derive(Debug, Clone, PartialEq, Eq)]
            pub struct Closing<'a>(&'a str);

            impl<'a> Closing<'a> {
                pub fn new(closing: &'a str) -> Self {
                    Self(closing)
                }
            }
        };
    }
    pub(in super::super) use closing_macro;

    macro_rules! closing_tests_macro {
        ($valid_closings:expr, $invalid:expr) => {
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
                fn should_fail_invalid_closing() {
                    assert_eq!(Err($invalid), closing($invalid));
                }

                #[test]
                fn should_work_for_valid_tag() {
                    for valid in $valid_closings {
                        assert_eq!(Ok(("", Closing::new(valid))), closing(valid),);
                    }
                }

                #[test]
                fn should_work_with_text_before_tag() {
                    for valid in $valid_closings {
                        let text = format!("some text before {}", valid);
                        assert_eq!(
                            Ok(("", Closing::new(text.as_str()))),
                            closing(text.as_str())
                        );
                    }
                }

                #[test]
                fn should_work_with_text_after_tag() {
                    for valid in $valid_closings {
                        let text = format!("{} some text after", valid);
                        assert_eq!(
                            Ok(("", Closing::new(text.as_str()))),
                            closing(text.as_str())
                        );
                    }
                }
            }
        };
    }
    pub(in super::super) use closing_tests_macro;

    macro_rules! closing_module_macro {
        ($predicate:expr, $valid_cosings:expr, $invalid:expr) => {
            mod closing {
                use crate::parse::blocks::block::macros::parse::{
                    closing_macro, closing_tests_macro,
                };

                closing_macro!($predicate);
                closing_tests_macro!($valid_cosings, $invalid);
            }
        };
    }
    pub(in super::super) use closing_module_macro;

    macro_rules! content_or_closing_macro {
        () => {
            content_or_closing_macro!(super);
        };
        ($($root:ident)::+) => {
            pub fn content_or_closing<'a>(
                line: &'a str,
            ) -> parser::ParseResult<&'a str, ContentOrClosing<'a>> {
                use parser::{Map, Parser};

                parser::one_of((
                    $($root)::+::closing.map(ContentOrClosing::from),
                    parser::rest.map(ContentOrClosing::from),
                ))
                .parse(line)
            }

            #[derive(Debug, Clone, PartialEq, Eq)]
            pub enum ContentOrClosing<'a> {
                Content(&'a str),
                Closing($($root)::+::Closing<'a>),
            }

            impl<'a> From<&'a str> for ContentOrClosing<'a> {
                fn from(content: &'a str) -> Self {
                    ContentOrClosing::Content(content)
                }
            }

            impl<'a> From<$($root)::+::Closing<'a>> for ContentOrClosing<'a> {
                fn from(closing: $($root)::+::Closing<'a>) -> Self {
                    ContentOrClosing::Closing(closing)
                }
            }
        };
    }
    pub(in super::super) use content_or_closing_macro;

    macro_rules! content_or_closing_tests_macro {
        ($content:expr, $closing:expr) => {
            #[cfg(test)]
            mod test {
                use super::super::Closing;
                use super::*;

                #[test]
                fn should_work_for_closing() {
                    assert_eq!(
                        Ok(("", ContentOrClosing::Closing(Closing::new($closing)))),
                        content_or_closing($closing),
                    );
                }

                #[test]
                fn should_default_to_content_when_not_closing() {
                    assert_eq!(
                        Ok(("", ContentOrClosing::Content($content))),
                        content_or_closing($content),
                    );
                }
            }
        };
    }
    pub(in super::super) use content_or_closing_tests_macro;

    macro_rules! content_or_closing_module_macro {
        ($content:expr, $closing:expr) => {
            mod content_or_closing {
                use crate::parse::blocks::block::macros::parse::{
                    content_or_closing_macro, content_or_closing_tests_macro,
                };

                content_or_closing_macro!();
                content_or_closing_tests_macro!($content, $closing);
            }
        };
    }
    pub(in super::super) use content_or_closing_module_macro;

    macro_rules! opening_maybe_closing_macro {
        () => {
            opening_maybe_closing_macro!(super);
        };
        ($($root:ident)::+) => {
            pub fn opening_maybe_closing(
                line: &str,
            ) -> parser::ParseResult<&str, OpeningMaybeClosing> {
                let (remaining, opening) = $($root)::+::opening(line)?;
                match $($root)::+::closing(line) {
                    Ok((remaining, _)) => Ok((remaining, OpeningAndClosing::new(line).into())),
                    Err(_) => Ok((remaining, opening.into())),
                }
            }

            #[derive(Debug, Clone, PartialEq, Eq)]
            pub enum OpeningMaybeClosing<'a> {
                Opening($($root)::+::Opening<'a>),
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

            impl<'a> From<$($root)::+::Opening<'a>> for OpeningMaybeClosing<'a> {
                fn from(opening: $($root)::+::Opening<'a>) -> Self {
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
        };
    }
    pub(in super::super) use opening_maybe_closing_macro;

    macro_rules! opening_maybe_closing_test_macro {
        ($opening:expr, $opening_and_closing:expr) => {
            #[cfg(test)]
            mod test {
                use super::super::opening;
                use super::*;
                use parser::StrictParse;

                #[test]
                fn should_work_for_opening() {
                    assert_eq!(
                        Ok((
                            "",
                            OpeningMaybeClosing::Opening(opening.strict_parse($opening))
                        )),
                        opening_maybe_closing($opening),
                    );
                }

                #[test]
                fn should_work_for_opening_and_closing() {
                    assert_eq!(
                        Ok((
                            "",
                            OpeningMaybeClosing::OpeningAndClosing(OpeningAndClosing::new(
                                $opening_and_closing
                            ))
                        )),
                        opening_maybe_closing($opening_and_closing),
                    );
                }
            }
        };
    }
    pub(in super::super) use opening_maybe_closing_test_macro;

    macro_rules! opening_maybe_closing_module_macro {
        ($opening:expr, $opening_and_closing:expr) => {
            mod opening_maybe_closing {
                use crate::parse::blocks::block::macros::parse::{
                    opening_maybe_closing_macro, opening_maybe_closing_test_macro,
                };

                opening_maybe_closing_macro!();
                opening_maybe_closing_test_macro!($opening, $opening_and_closing);
            }
        };
    }
    pub(in super::super) use opening_maybe_closing_module_macro;
}
