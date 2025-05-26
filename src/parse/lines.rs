use super::input::Input;
use parser::{Indexable, IsEmpty, ItemsIndices, PrefixEnd, SplitAt, SubsetRange};
use std::str::{CharIndices, SplitInclusive};

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

impl<'a> ItemsIndices<&'a str> for Lines<'a> {
    type ItemsIndices = LinesIndices<SplitInclusive<'a, char>>;

    fn items_indices(&self) -> Self::ItemsIndices {
        // This iterator outputs line by line and the index is the byte offset of the first character of the line.
        LinesIndices::from(*self)
    }
}

impl<'a> ItemsIndices<char> for Lines<'a> {
    type ItemsIndices = CharIndices<'a>;

    fn items_indices(&self) -> Self::ItemsIndices {
        self.source.items_indices()
    }
}

impl<'a> PrefixEnd<&'a str> for Lines<'a> {
    fn prefix_end(&self, tag: &'a str) -> Option<Self::Index> {
        self.source.prefix_end(tag)
    }
}

impl<'a> SubsetRange<Self> for Lines<'a> {
    fn subset_range(&self, item: Self) -> (Self::Index, Self::Index) {
        self.subset_range(item.source)
    }
}

impl<'a> SubsetRange<&'a str> for Lines<'a> {
    fn subset_range(&self, item: &'a str) -> (Self::Index, Self::Index) {
        self.source.subset_range(item)
    }
}

impl SplitAt for Lines<'_> {
    fn split_at(&self, index: Self::Index) -> (Self, Self) {
        let (left, right) = self.source.split_at(index);
        (left.into(), right.into())
    }
}

impl IsEmpty for Lines<'_> {
    fn is_empty(&self) -> bool {
        self.source.is_empty()
    }
}

impl<'a> Input<'a> for Lines<'a> {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LinesIndices<I> {
    iter: I,
    offset: usize,
}

impl<I> LinesIndices<I> {
    fn new(iter: I) -> Self {
        Self { iter, offset: 0 }
    }
}

impl<'a> From<Lines<'a>> for LinesIndices<SplitInclusive<'a, char>> {
    fn from(lines: Lines<'a>) -> Self {
        Self::new(lines.source.split_inclusive('\n'))
    }
}

impl<'a, I> Iterator for LinesIndices<I>
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
