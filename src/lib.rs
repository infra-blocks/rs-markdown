// TODO: mark all the segment and struct constructors as unsafe and make sure they are not public outside this module.

pub(crate) mod parse;
pub mod render;

use nom::error::Error;
pub use parse::ast::Tree;
use parse::traits::ParseWhole;
pub use render::to_html::ToHtml;

pub fn parse(input: &str) -> Tree {
    Tree::parse_whole::<Error<&str>>(input).expect("unexpected error parsing markdown")
}
