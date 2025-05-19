use super::ParseResult;

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
    Self: Sized + Clone,
{
    type Item;

    /// Returns whether the input is empty.
    fn is_empty(&self) -> bool;

    /// Returns the input with the given quantity consumed.
    fn consumed(&self, quantity: ParseQuantity) -> Self;

    /// Returns the first item of the input.
    ///
    /// An empty string signifies the end of the input.
    fn first(&self) -> Option<Self::Item> {
        self.items().next()
    }

    /// Returns an iterator over the items of the input.
    fn items(&self) -> impl Iterator<Item = Self::Item>;

    /// Returns a successful parse result, stripping the input of the given quantity parsed.
    fn parsed<T>(self, quantity: ParseQuantity, value: T) -> ParseResult<Self, T>
    where
        T: Sized,
    {
        let remaining = self.consumed(quantity);
        Ok((remaining, value))
    }

    /// Returns a failed parse result, returning the input untouched.
    fn failed<T>(self) -> ParseResult<Self, T>
    where
        T: Sized,
    {
        Err(self)
    }
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

impl<'a> Input for &'a str {
    type Item = &'a str;

    fn is_empty(&self) -> bool {
        str::is_empty(self)
    }

    fn consumed(&self, quantity: ParseQuantity) -> Self {
        &self[self.quantity_in_bytes(quantity)..]
    }

    fn items(&self) -> impl Iterator<Item = Self::Item> {
        self.split_inclusive("\n")
    }
}
