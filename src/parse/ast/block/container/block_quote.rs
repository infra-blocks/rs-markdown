use crate::parse::parsers::indented_by_less_than_4;
use parser::{ParseResult, Parser, maybe, recognize, tag};

pub fn marker(input: &str) -> ParseResult<&str, &str> {
    // A block quote marker is a `>` followed by optional whitespace.
    recognize((indented_by_less_than_4, tag(">"), maybe(tag(" ")))).parse(input)
}
