use super::Parsable;

pub enum ParseQuantity {
    /// The quantity of items parsed.
    Items(usize),
    // TODO: this should be a bound on the item of input. It should be able to "consume" itself for a given number of bytes.
    /// The quantity of bytes parsed.
    Bytes(usize),
}

/// A trait to regroup different input types.
///
/// Some frequent algorithms like to walk back on inputs. To do so, we could implement rewinding
/// semantics here, but for the sake of simplicity, we decided to first start with using the [Clone]
/// trait. This is why [Input] are also expected to be [Clone]. That being said, in the context of
/// this program, [Input]s are lightweight and they also implement [Copy], making the clone operation
/// quite cheap.
pub trait Input
where
    Self: Parsable<Quantity = ParseQuantity> + Clone,
{
    type Item;

    /// Returns whether the input is empty.
    fn is_empty(&self) -> bool;

    /// Returns the first item of the input.
    ///
    /// An empty string signifies the end of the input.
    fn first(&self) -> Option<Self::Item> {
        self.items().next()
    }

    /// Returns an iterator over the items of the input.
    fn items(&self) -> impl Iterator<Item = Self::Item>;
}
