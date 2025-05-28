mod atx_heading;
mod blank_line;
mod fenced_code;
mod html;
mod indented_code;
mod link_reference_definition;
mod thematic_break;

use crate::{
    ast::block::{Leaf, LinkReferenceDefinition},
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
            Leaf::FencedCode(fenced_code) => {
                fenced_code.display_html(buffer, link_reference_definitions)
            }
            Leaf::Html(html) => html.display_html(buffer, link_reference_definitions),
            Leaf::IndentedCode(indented_code) => {
                indented_code.display_html(buffer, link_reference_definitions)
            }
            Leaf::LinkReferenceDefinition(link_reference_definition) => {
                link_reference_definition.display_html(buffer, link_reference_definitions)
            }
            Leaf::ThematicBreak(thematic_break) => {
                thematic_break.display_html(buffer, link_reference_definitions)
            }
        }
    }
}
