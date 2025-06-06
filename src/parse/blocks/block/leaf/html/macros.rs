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

pub mod open {
    macro_rules! html_case_macro {
        ($type:ident, $($root:ident)::+) => {
            #[derive(Debug, Clone, PartialEq, Eq)]
            pub enum $type<'a> {
                SingleLine(single::SingleLine<'a>),
                MultiLine(multi::MultiLine<'a>),
            }

            impl<'a> $type<'a> {
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

            impl<'a> crate::parse::blocks::open_block::IOpenBlock<'a> for $type<'a> {
                type Closed = $($root)::+::$type<'a>;

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

    macro_rules! single_line_macro {
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

    macro_rules! content_or_closing_macro {
        ($closing:ident < $lifetime:tt >) => {
            pub fn content_or_closing<$lifetime>(
                line: &$lifetime str,
            ) -> parser::ParseResult<&$lifetime str, ContentOrClosing<$lifetime>> {
                use parser::{Map, Parser};

                parser::one_of((
                    closing.map(ContentOrClosing::from),
                    parser::rest.map(ContentOrClosing::from),
                ))
                .parse(line)
            }

            #[derive(Debug, Clone, PartialEq, Eq)]
            pub enum ContentOrClosing<$lifetime> {
                Content(&$lifetime str),
                Closing($closing<$lifetime>),
            }

            impl<$lifetime> From<&$lifetime str> for ContentOrClosing<$lifetime> {
                fn from(content: &$lifetime str) -> Self {
                    ContentOrClosing::Content(content)
                }
            }

            impl<$lifetime> From<$closing<$lifetime>> for ContentOrClosing<$lifetime> {
                fn from(closing: Closing<$lifetime>) -> Self {
                    ContentOrClosing::Closing(closing)
                }
            }
        };
    }
    pub(in super::super) use content_or_closing_macro;

    macro_rules! opening_maybe_closing_macro {
        ($opening:ident < $lifetime:tt >, $parse_opening:expr, $parse_closing: expr) => {
            pub fn opening_maybe_closing(
                line: &str,
            ) -> parser::ParseResult<&str, OpeningMaybeClosing> {
                let (remaining, opening) = $parse_opening(line)?;
                match $parse_closing(line) {
                    Ok((remaining, _)) => Ok((remaining, OpeningAndClosing::new(line).into())),
                    Err(_) => Ok((remaining, opening.into())),
                }
            }

            #[derive(Debug, Clone, PartialEq, Eq)]
            pub enum OpeningMaybeClosing<'a> {
                Opening($opening<'a>),
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

            impl<'a> From<$opening<'a>> for OpeningMaybeClosing<'a> {
                fn from(opening: $opening<'a>) -> Self {
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
}
