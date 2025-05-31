use crate::{
    ast::block::{Container, LinkReferenceDefinition},
    render::display_html::DisplayHtml,
};

impl DisplayHtml for Container<'_> {
    fn display_html(&self, _: &mut String, _: &[LinkReferenceDefinition]) {
        unimplemented!()
    }
}
