use crate::{
    ast::block::{Block, LinkReferenceDefinition},
    render::display_html::DisplayHtml,
};

mod container;
mod leaf;

impl DisplayHtml for Block<'_> {
    fn display_html(
        &self,
        buffer: &mut String,
        link_reference_definitions: &[LinkReferenceDefinition],
    ) {
        match self {
            Block::Container(container) => {
                container.display_html(buffer, link_reference_definitions)
            }
            Block::Leaf(leaf) => leaf.display_html(buffer, link_reference_definitions),
        }
    }
}
