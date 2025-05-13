use crate::{
    inline::link::ParenthesesLinkTitle,
    parse::{segment::link_title::ParenthesesLinkTitleSegments, traits::Parse},
};
use nom::{IResult, Parser, error::ParseError};

impl<'a> Parse<'a> for ParenthesesLinkTitle<'a> {
    fn parse<Error: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, Self, Error>
    where
        Self: Sized,
    {
        ParenthesesLinkTitleSegments::parse
            .map(Self::new)
            .parse(input)
    }
}
