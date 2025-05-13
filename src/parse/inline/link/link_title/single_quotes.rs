use crate::{
    inline::link::SingleQuotesLinkTitle,
    parse::{segment::link_title::SingleQuotesLinkTitleSegments, traits::Parse},
};
use nom::{IResult, Parser, error::ParseError};

impl<'a> Parse<'a> for SingleQuotesLinkTitle<'a> {
    fn parse<Error: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, Self, Error>
    where
        Self: Sized,
    {
        SingleQuotesLinkTitleSegments::parse
            .map(Self::new)
            .parse(input)
    }
}
