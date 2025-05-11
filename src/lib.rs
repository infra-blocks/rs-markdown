// TODO: mark all the segment and struct constructors as unsafe and make sure they are not public outside this module.

pub(crate) mod parse;
pub mod render;

pub use parse::ast::Tree;
use parse::input::strict_parse;
pub use render::to_html::ToHtml;

pub fn parse(input: &str) -> Tree {
    strict_parse(input)
}
