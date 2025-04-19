mod atx_heading;

use atx_heading::AtxHeading;
use nom::Parser;

pub enum Leaf<'a> {
    AtxHeading(AtxHeading<'a>),
}

impl<'a> Leaf<'a> {
    pub fn parser<Error: nom::error::ParseError<&'a str>>(
    ) -> impl nom::Parser<&'a str, Output = Self, Error = Error> {
        AtxHeading::parser().map(Leaf::AtxHeading)
    }
}
