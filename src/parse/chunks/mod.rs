mod chunks_quantity;
mod line_chunks;

pub use chunks_quantity::*;
pub use line_chunks::*;

use super::{Consume, chunk::Chunk};

pub trait Chunks: Consume<Quantity = ChunksQuantity>
where
    Self: Sized,
{
    type Chunk: Chunk;

    /// Returns whether there are chunks left to parse.
    fn is_empty(&self) -> bool {
        self.chunks().next().is_none()
    }

    /// Returns the first item of the input.
    ///
    /// An empty string signifies the end of the input.
    fn first(&self) -> Option<Self::Chunk> {
        self.chunks().next()
    }

    /// Returns an iterator over the items of the input.
    fn chunks(&self) -> impl Iterator<Item = Self::Chunk>;
}
