mod api;
pub(crate) mod parse;
pub mod render;

use api::ast::Tree;
pub use api::*;
use parse::traits::StrictParse;

pub fn parse(input: &str) -> Tree {
    Tree::strict_parse(input)
}
