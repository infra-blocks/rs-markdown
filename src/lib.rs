mod api;
pub(crate) mod parse;
pub mod render;

use api::ast::Tree;
pub use api::*;
use parse::{lines, parser::IsEmpty, traits::Parse};

pub fn parse(input: &str) -> Tree {
    let (remaining, parsed) = Tree::parse(lines(input)).expect("unexpected error parsing markdown");
    if !remaining.is_empty() {
        panic!("unexpected remaining input: {remaining:?}");
    }
    parsed
}
