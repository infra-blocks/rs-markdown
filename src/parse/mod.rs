pub mod ast;
mod blocks;
pub mod input;
mod lines;
pub mod parsers;
pub mod predicates;
pub mod segment;
#[cfg(test)]
pub mod test_utils;
pub mod traits;

use crate::api;
pub use lines::*;

pub fn parse(input: &str) -> api::ast::Document {
    unimplemented!("document parse unimplemented");
}
