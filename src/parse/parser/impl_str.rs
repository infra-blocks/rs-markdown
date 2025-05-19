use super::{Enumerate, Index, Indexable, SplitAt};

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
