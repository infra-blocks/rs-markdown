mod atx_heading;
mod blank_line;

use atx_heading::AtxHeading;
use blank_line::BlankLine;
use nom::{branch::alt, Parser};

pub enum Leaf<'a> {
    AtxHeading(AtxHeading<'a>),
    BlankLine(BlankLine<'a>),
}

impl<'a> Leaf<'a> {
    pub fn parser<Error: nom::error::ParseError<&'a str>>(
    ) -> impl nom::Parser<&'a str, Output = Self, Error = Error> {
        alt((
            AtxHeading::parser().map(Leaf::AtxHeading),
            BlankLine::parser().map(Leaf::BlankLine),
        ))
    }
}
