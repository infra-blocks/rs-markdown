use super::{Enumerate, Index, Indexable, IsEmpty, PrefixEnd, SplitAt, SubsetRange};

impl Index for usize {}

impl Indexable for &str {
    type Index = usize;

    fn last_index(&self) -> Self::Index {
        self.len()
    }
}

impl Enumerate<char> for &str {
    fn items_indices(&self) -> impl Iterator<Item = (Self::Index, char)> {
        self.char_indices()
    }
}

impl SplitAt for &str {
    fn split_at(&self, index: Self::Index) -> (Self, Self) {
        (self as &str).split_at(index)
    }
}

impl PrefixEnd<char> for &str {
    fn prefix_end(&self, tag: char) -> Option<Self::Index> {
        if self.chars().next()? == tag {
            return Some(1);
        }
        None
    }
}

impl PrefixEnd<&str> for &str {
    fn prefix_end(&self, tag: &str) -> Option<Self::Index> {
        if self.starts_with(tag) {
            return Some(tag.len());
        }
        None
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
