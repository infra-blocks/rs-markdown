use crate::parse::ast::block::leaf::link_reference_definition::LinkReferenceDefinition;

pub(crate) trait DisplayHtml {
    fn display_html(
        &self,
        buffer: &mut String,
        link_reference_definitions: &[LinkReferenceDefinition],
    );
}
