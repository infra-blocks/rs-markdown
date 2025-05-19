use super::Chunk;
use crate::parse::ParseResult;

pub trait ChunkParser<C>
where
    C: Chunk,
{
    type Output;

    fn parse(&self, chunk: C) -> ParseResult<C, Self::Output>;
}

impl<C, O, F> ChunkParser<C> for F
where
    F: Fn(C) -> ParseResult<C, O>,
    C: Chunk,
{
    type Output = O;

    fn parse(&self, chunk: C) -> ParseResult<C, Self::Output> {
        (self)(chunk)
    }
}
