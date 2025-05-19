use super::Chunk;
use crate::parse::Consume;

impl<'a, T> Consume for &'a [T] {
    type Quantity = usize;

    fn consume(self, quantity: Self::Quantity) -> Self {
        &self[quantity..]
    }
}

impl<'a, T> Chunk for &'a [T] {
    type Item = &'a T;

    fn item_indices(&self) -> impl Iterator<Item = (usize, Self::Item)> {
        self.iter().enumerate()
    }

    fn len(&self) -> usize {
        (self as &[T]).len()
    }
}
