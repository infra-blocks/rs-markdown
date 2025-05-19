use crate::inline::link::{LinkDestination, LinkLabel, LinkTitle};

/// Link reference definition, as described in the [spec](https://spec.commonmark.org/0.31.2/#link-reference-definitions)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LinkReferenceDefinition<'a> {
    segments: Vec<&'a str>,
    label: LinkLabel<'a>,
    destination: LinkDestination<'a>,
    title: Option<LinkTitle<'a>>,
}

impl<'a> LinkReferenceDefinition<'a> {
    /// Creates a new link reference definition.
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
