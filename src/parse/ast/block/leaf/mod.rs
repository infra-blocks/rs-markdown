pub mod atx_heading;
pub mod blank_line;
pub mod fenced_code;
pub mod indented_code;
pub mod thematic_break;

use crate::{
    ast::{AtxHeading, BlankLine, FencedCode, IndentedCode, Leaf, ThematicBreak},
    parse::traits::Parse,
};
use nom::{Parser, branch::alt};

impl<'a> Parse<'a> for Leaf<'a> {
    fn parse<Error: nom::error::ParseError<&'a str>>(
        input: &'a str,
    ) -> nom::IResult<&'a str, Self, Error> {
        alt((
            AtxHeading::parse.map(Leaf::AtxHeading),
            BlankLine::parse.map(Leaf::BlankLine),
            FencedCode::parse.map(Leaf::FencedCode),
            IndentedCode::parse.map(Leaf::IndentedCode),
            ThematicBreak::parse.map(Leaf::ThematicBreak),
        ))
        .parse(input)
    }
}
