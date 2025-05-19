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

trait StrInput<'a> {
    fn bytes_for_items(&self, count: usize) -> usize;
    fn quantity_in_bytes(&self, quantity: ParseQuantity) -> usize;
}

impl<'a> StrInput<'a> for &'a str {
    fn bytes_for_items(&self, count: usize) -> usize {
        let mut index = 0;
        let mut bytes = 0;
        for segment in self.items() {
            bytes += segment.len();
            index += 1;
            if index == count {
                return bytes;
            }
        }
        panic!("invalid segment count {count} for input {self}");
    }

    fn quantity_in_bytes(&self, quantity: ParseQuantity) -> usize {
        match quantity {
            ParseQuantity::Items(count) => self.bytes_for_items(count),
            // Validate bytes
            ParseQuantity::Bytes(count) => {
                assert!(
                    count >= 1 && count <= self.len(),
                    "invalid byte count {} for input {}, expected range [{}, {}]",
                    count,
                    self,
                    1,
                    self.len()
                );
                count
            }
        }
    }
}

impl Parsable for &'_ str {
    type Quantity = ParseQuantity;

    fn consume(self, quantity: Self::Quantity) -> Self {
        &self[self.quantity_in_bytes(quantity)..]
    }
}

impl<'a> Input for &'a str {
    type Item = &'a str;

    fn is_empty(&self) -> bool {
        str::is_empty(self)
    }

    fn items(&self) -> impl Iterator<Item = Self::Item> {
        self.split_inclusive("\n")
    }
}
