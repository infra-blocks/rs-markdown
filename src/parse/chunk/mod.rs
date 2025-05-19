mod parser;
mod slice_chunk;
mod str_chunk;

use super::Consume;
pub use parser::*;
pub use slice_chunk::*;
pub use str_chunk::*;

/// A trait for a parsable chunk.
///
/// A chunk is a contiguous sequence of items that can be randomly accessed
/// using indices. Furthermore, they are expected to be splittable. Meaning,
/// we can split the chunk at a given index to obtain two new chunks.
pub trait Chunk: Consume<Quantity = usize> {
    type Item;

    fn item_indices(&self) -> impl Iterator<Item = (usize, Self::Item)>;
    fn len(&self) -> usize;
}
