pub(crate) mod parse;

use nom::{error::Error, Parser};
pub use parse::ast::Tree;

// TODO: custom error type.
pub fn parse(input: &str) -> Result<Tree, nom::Err<Error<&str>>> {
    let mut parser = Tree::parser::<Error<&str>>();
    let (remaining, tree) = parser.parse(input)?;
    assert!(remaining.is_empty(), "Remaining input: {}", remaining);
    Ok(tree)
}
