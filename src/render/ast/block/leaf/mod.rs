mod atx_heading;
mod blank_line;
mod thematic_break;

use crate::{
    parse::ast::block::leaf::{link_reference_definition::LinkReferenceDefinition, Leaf},
    render::display_html::DisplayHtml,
};

impl DisplayHtml for Leaf<'_> {
    fn display_html(
        &self,
        buffer: &mut String,
        link_reference_definitions: &[LinkReferenceDefinition],
    ) {
        match self {
            Leaf::AtxHeading(atx_heading) => {
                atx_heading.display_html(buffer, link_reference_definitions)
            }
            Leaf::BlankLine(blank_line) => {
                blank_line.display_html(buffer, link_reference_definitions)
            }
            Leaf::ThematicBreak(thematic_break) => {
                thematic_break.display_html(buffer, link_reference_definitions)
            }
            _ => unimplemented!("diplay html not implemented for {:?}", self),
        }
    }
}
