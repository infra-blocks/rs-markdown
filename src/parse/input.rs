pub enum ParseQuantity {
    /// The quantity of segments parsed.
    Segments(usize),
    /// The quantity of bytes parsed.
    Bytes(usize),
}

pub type ParseResult<I, T> = Result<(I, T), I>;

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

    // TODO: rename this for item.
    // TODO: return an option and None if the input is empty. It's clearer.
    /// Returns the first segment of the input.
    ///
    /// An empty string signifies the end of the input.
    fn segment(&self) -> Self::Item;

    /// Returns an iterator over the segments of the input.
    fn segments(&self) -> impl Iterator<Item = Self::Item>;

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
    fn bytes_for_segments(&self, count: usize) -> usize;
    fn quantity_in_bytes(&self, quantity: ParseQuantity) -> usize;
}

impl<'a> StrInput<'a> for &'a str {
    fn bytes_for_segments(&self, count: usize) -> usize {
        let mut index = 0;
        let mut bytes = 0;
        for segment in self.segments() {
            bytes += segment.len();
            index += 1;
            if index == count {
                return bytes;
            }
        }
        panic!("invalid segment count {} for input {}", count, self);
    }

    fn quantity_in_bytes(&self, quantity: ParseQuantity) -> usize {
        match quantity {
            ParseQuantity::Segments(count) => self.bytes_for_segments(count),
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

    fn segment(&self) -> Self::Item {
        self.segments().next().unwrap_or("")
    }

    fn segments(&self) -> impl Iterator<Item = Self::Item> {
        self.split_inclusive("\n")
    }
}
