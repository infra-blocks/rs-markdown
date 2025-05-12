use crate::Segment;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BlankLine<'a>(&'a str);

impl<'a> BlankLine<'a> {
    pub(crate) fn new(segment: &'a str) -> Self {
        Self(segment)
    }
}

impl<'a> Segment<'a> for BlankLine<'a> {
    fn segment(&self) -> &'a str {
        self.0
    }
}
