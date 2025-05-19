use super::Chunk;
use crate::parse::Consume;

impl<'a> Consume for &'a str {
    type Quantity = usize;

    fn consume(self, count: usize) -> Self {
        &self[count..]
    }
}

impl<'a> Chunk for &'a str {
    type Item = char;

    fn item_indices(&self) -> impl Iterator<Item = (usize, Self::Item)> {
        self.char_indices()
    }

    fn len(&self) -> usize {
        (self as &str).len()
    }
}
