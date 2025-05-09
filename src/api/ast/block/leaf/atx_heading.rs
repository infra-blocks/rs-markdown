use crate::Segment;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AtxHeading<'a> {
    /// The source segment from which this struct was constructed.
    segment: &'a str,
    /// The title of the heading, possibly empty.
    title: &'a str,
    /// The level of the heading, from 1 to 6.
    level: u8,
}

impl<'a> AtxHeading<'a> {
    pub(crate) fn new(segment: &'a str, title: &'a str, level: u8) -> Self {
        Self {
            segment,
            title,
            level,
        }
    }

    pub fn level(&self) -> u8 {
        self.level
    }

    pub fn title(&self) -> &'a str {
        self.title
    }
}

impl<'a> Segment<'a> for AtxHeading<'a> {
    fn segment(&self) -> &'a str {
        self.segment
    }
}
