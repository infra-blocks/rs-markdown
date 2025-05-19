use super::{Chunks, ChunksQuantity};
use crate::parse::Consume;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LineChunks<'a> {
    source: &'a str,
}

impl<'a> LineChunks<'a> {
    pub fn new(source: &'a str) -> Self {
        Self { source }
    }

    fn bytes_for_chunks(&self, count: usize) -> usize {
        let mut index = 0;
        let mut bytes = 0;
        for chunk in self.chunks() {
            bytes += chunk.len();
            index += 1;
            if index == count {
                return bytes;
            }
        }
        panic!("invalid chunks count {count} for input {self:?}");
    }

    fn quantity_in_bytes(&self, quantity: ChunksQuantity) -> usize {
        self.bytes_for_chunks(quantity.chunks_count()) + quantity.items_count()
    }
}

impl<'a> Consume for LineChunks<'a> {
    type Quantity = ChunksQuantity;

    fn consume(self, count: Self::Quantity) -> Self {
        Self::from(&self.source[self.quantity_in_bytes(count)..])
    }
}

impl<'a> Chunks for LineChunks<'a> {
    type Chunk = &'a str;

    fn chunks(&self) -> impl Iterator<Item = Self::Chunk> {
        self.source.split_inclusive('\n')
    }
}

impl<'a> From<&'a str> for LineChunks<'a> {
    fn from(source: &'a str) -> Self {
        Self::new(source)
    }
}
