use crate::Segment;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ThematicBreak<'a>(&'a str);

impl<'a> ThematicBreak<'a> {
    pub(crate) fn new(segment: &'a str) -> Self {
        Self(segment)
    }
}

impl<'a> Segment<'a> for ThematicBreak<'a> {
    fn segment(&self) -> &'a str {
        self.0
    }
}
