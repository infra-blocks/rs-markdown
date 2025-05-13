use crate::{
    inline::link::DoubleQuotesLinkTitle,
    parse::{segment::link_title::DoubleQuotesLinkTitleSegments, traits::Parse},
};
use nom::{IResult, Parser, error::ParseError};

impl<'a> Parse<'a> for DoubleQuotesLinkTitle<'a> {
    fn parse<Error: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, Self, Error>
    where
        Self: Sized,
    {
        DoubleQuotesLinkTitleSegments::parse
            .map(Self::new)
            .parse(input)
    }
}
