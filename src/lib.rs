mod api;
pub(crate) mod build;
pub(crate) mod parse;
pub mod render;

use api::ast::Document;
pub use api::*;
use parse::{lines, traits::Parse};
use parser::IsEmpty;

pub fn parse(input: &str) -> Document {
    let (remaining, parsed) =
        Document::parse(lines(input)).expect("unexpected error parsing markdown");
    if !remaining.is_empty() {
        panic!("unexpected remaining input: {remaining:?}");
    }
    parsed
}
