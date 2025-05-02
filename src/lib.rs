pub(crate) mod parse;
pub mod render;

use nom::{error::Error, Parser};
pub use parse::ast::Tree;
pub use render::to_html::ToHtml;

pub fn parse(input: &str) -> Tree {
    let mut parser = Tree::parser::<Error<&str>>();
    let (remaining, tree) = parser
        .parse(input)
        .expect("unexpected error parsing markdown");
    assert!(remaining.is_empty(), "Remaining input: {}", remaining);
    tree
}
