use crate::ast::inline::link::{LinkDestination, LinkLabel, LinkTitle};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LinkReferenceDefinition<'a> {
    segments: Vec<&'a str>,
    label: LinkLabel<'a>,
    destination: LinkDestination<'a>,
    title: Option<LinkTitle<'a>>,
}

impl<'a> LinkReferenceDefinition<'a> {
    pub(crate) fn new(
        segments: Vec<&'a str>,
        label: LinkLabel<'a>,
        destination: LinkDestination<'a>,
        title: Option<LinkTitle<'a>>,
    ) -> Self {
        Self {
            segments,
            label,
            destination,
            title,
        }
    }
}
