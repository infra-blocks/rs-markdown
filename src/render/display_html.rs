use crate::ast::LinkReferenceDefinition;

pub(crate) trait DisplayHtml {
    fn display_html(
        &self,
        buffer: &mut String,
        link_reference_definitions: &[LinkReferenceDefinition],
    );
}
