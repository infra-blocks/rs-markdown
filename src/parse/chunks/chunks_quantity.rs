pub struct ChunksQuantity {
    /// Amount of whole chunks.
    chunks_count: usize,
    /// Amount of items in the last chunk.
    items_count: usize,
}

impl ChunksQuantity {
    pub fn new(chunks_count: usize, items_count: usize) -> Self {
        Self {
            chunks_count,
            items_count,
        }
    }

    pub fn chunks_count(&self) -> usize {
        self.chunks_count
    }

    pub fn items_count(&self) -> usize {
        self.items_count
    }
}
