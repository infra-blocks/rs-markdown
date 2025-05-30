use crate::{
    Segments,
    ast::block::{Html, LinkReferenceDefinition},
    render::DisplayHtml,
};

impl DisplayHtml for Html<'_> {
    /// HTML blocks are not escaped.
    fn display_html(
        &self,
        buffer: &mut String,
        _link_reference_definitions: &[LinkReferenceDefinition],
    ) {
        for segment in self.segments() {
            buffer.push_str(segment);
        }
    }
}
