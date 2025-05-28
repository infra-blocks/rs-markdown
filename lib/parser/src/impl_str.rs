use super::{Index, Indexable, IsEmpty, ItemsIndices, SplitAt, SubsetRange};
use std::str::CharIndices;

impl Index for usize {}

impl Indexable for &str {
    type Index = usize;

    fn last_index(&self) -> Self::Index {
        self.len()
    }
}

impl<'a> ItemsIndices<char> for &'a str {
    type ItemsIndices = CharIndices<'a>;

    fn items_indices(&self) -> Self::ItemsIndices {
        self.char_indices()
    }
}

impl SplitAt for &str {
    fn split_at(&self, index: Self::Index) -> (Self, Self) {
        (self as &str).split_at(index)
    }
}

impl SubsetRange<&str> for &str {
    fn subset_range(&self, item: &str) -> (Self::Index, Self::Index) {
        // TODO: use substr_range when stabilized.
        let source_start = self.as_ptr() as usize;
        let source_end = source_start + self.len();
        let item_start = item.as_ptr() as usize;
        let item_end = item_start + item.len();
        if item_start < source_start || item_end > source_end {
            panic!("item {item} not part of this input {self:?}");
        }
        (item_start - source_start, item_end - source_start)
    }
}

impl IsEmpty for &str {
    fn is_empty(&self) -> bool {
        (self as &str).is_empty()
    }
}
