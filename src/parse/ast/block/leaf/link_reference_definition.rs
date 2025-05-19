use crate::{
    Segment,
    ast::LinkReferenceDefinition,
    inline::link::{LinkDestination, LinkLabel, LinkTitle},
    parse::{
        input::{Input, ParseResult},
        traits::Parse,
    },
};

impl<'a> Parse<&'a str> for LinkReferenceDefinition<'a> {
    fn parse<I: Input<Item = &'a str>>(input: I) -> ParseResult<I, Self> {
        unimplemented!()
    }
}

// TODO: move to segments.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LinkReferenceDefinitionOpeningSegment<'a> {
    segment: &'a str,
    label: LinkLabel<'a>,
    destination: Option<LinkDestination<'a>>,
    title: Option<LinkTitle<'a>>,
}

impl<'a> Parse<&'a str> for LinkReferenceDefinitionOpeningSegment<'a> {
    fn parse<I: Input<Item = &'a str>>(input: I) -> ParseResult<I, Self> {
        unimplemented!()
    }
}

impl<'a> Segment<'a> for LinkReferenceDefinitionOpeningSegment<'a> {
    fn segment(&self) -> &'a str {
        self.segment
    }
}
