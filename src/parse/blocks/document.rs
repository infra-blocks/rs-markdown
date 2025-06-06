use super::block::Block;
use crate::ast::block::LinkReferenceDefinition;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Document<'a> {
    blocks: Vec<Block<'a>>,
    link_reference_definitions: Vec<LinkReferenceDefinition<'a>>,
}
