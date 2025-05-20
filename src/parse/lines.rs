use super::{
    input::{IndexOf, Input},
    parser::{Enumerate, Indexable, SplitAt},
};
use std::str::SplitInclusive;

pub fn lines<'a, T: Into<Lines<'a>>>(source: T) -> Lines<'a> {
    source.into()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Lines<'a> {
    source: &'a str,
}

impl<'a> Lines<'a> {
    pub fn new(source: &'a str) -> Self {
        Self { source }
    }

    pub fn len(&self) -> usize {
        self.source.len()
    }
}

impl<'a> From<&'a str> for Lines<'a> {
    fn from(source: &'a str) -> Self {
        Self::new(source)
    }
}

impl Indexable for Lines<'_> {
    type Index = usize;

    fn last_index(&self) -> Self::Index {
        self.len()
    }
}

impl<'a> Enumerate<&'a str> for Lines<'a> {
    fn items_indices(&self) -> impl Iterator<Item = (Self::Index, &'a str)> {
        // Careful to return the byte offset of the line with the line.
        LinesEnumerator::from(*self)
    }
}

impl<'a> IndexOf<&'a str> for Lines<'a> {
    fn index_of(&self, item: &'a str) -> Self::Index {
        let source_start = self.source.as_ptr() as usize;
        let source_end = source_start + self.len();
        let item_start = item.as_ptr() as usize;
        let item_end = item_start + item.len();
        if item_start < source_start || item_end > source_end {
            panic!("item {item} not part of this input {self:?}");
        }
        item_start - source_start
    }
}

impl SplitAt for Lines<'_> {
    fn split_at(&self, index: Self::Index) -> (Self, Self) {
        let (left, right) = self.source.split_at(index);
        (left.into(), right.into())
    }
}

impl<'a> Input<&'a str> for Lines<'a> {}

struct LinesEnumerator<I> {
    iter: I,
    offset: usize,
}

impl<I> LinesEnumerator<I> {
    fn new(iter: I) -> Self {
        Self { iter, offset: 0 }
    }
}

impl<'a> From<Lines<'a>> for LinesEnumerator<SplitInclusive<'a, char>> {
    fn from(lines: Lines<'a>) -> Self {
        Self::new(lines.source.split_inclusive('\n'))
    }
}

impl<'a, I> Iterator for LinesEnumerator<I>
where
    I: Iterator<Item = &'a str>,
{
    type Item = (usize, &'a str);

    fn next(&mut self) -> Option<Self::Item> {
        let line = self.iter.next()?;
        let line_len = line.len();
        // The offset currently stored is the right one for this line.
        let tuple = (self.offset, line);
        // Update the offset for the next line.
        self.offset += line_len;
        Some(tuple)
    }
}
