//TODO: complete this struct.
//TODO: to_html() for this struct won't produce anything.
// Only its usages in the rest of the document would.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LinkReferenceDefinition<'a> {
    _phantom: std::marker::PhantomData<&'a ()>,
}
