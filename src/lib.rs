mod api;
pub(crate) mod parse;
pub mod render;

use api::ast::Tree;
pub use api::*;
use nom::error::Error;
use parse::traits::ParseWhole;

pub fn parse(input: &str) -> Tree {
    Tree::parse_whole::<Error<&str>>(input).expect("unexpected error parsing markdown")
}
