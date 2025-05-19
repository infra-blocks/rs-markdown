use super::{
    Parsable,
    input::{Input, ParseQuantity},
};

macro_rules! lines {
    ($source:expr) => {
        crate::parse::Lines::from($source)
    };
}
pub(crate) use lines;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Lines<'a> {
    source: &'a str,
}

impl<'a> Lines<'a> {
    pub fn new(source: &'a str) -> Self {
        Self { source }
    }

    pub fn len(&self) -> usize {
        self.source.len()
    }

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
        panic!("invalid segment count {count} for input {self:?}");
    }

    fn quantity_in_bytes(&self, quantity: ParseQuantity) -> usize {
        match quantity {
            ParseQuantity::Items(count) => self.bytes_for_items(count),
            // Validate bytes
            ParseQuantity::Bytes(count) => {
                assert!(
                    count <= self.len(),
                    "invalid byte count {count} for input {self:?}, expected range [0, {}]",
                    self.len()
                );
                count
            }
        }
    }
}

impl<'a> From<&'a str> for Lines<'a> {
    fn from(source: &'a str) -> Self {
        Self::new(source)
    }
}

impl<'a> Parsable for Lines<'a> {
    type Quantity = ParseQuantity;

    fn consume(self, quantity: Self::Quantity) -> Self {
        (&self.source[self.quantity_in_bytes(quantity)..]).into()
    }
}

impl<'a> Input for Lines<'a> {
    type Item = &'a str;

    fn is_empty(&self) -> bool {
        self.source.is_empty()
    }

    fn items(&self) -> impl Iterator<Item = Self::Item> {
        self.source.split_inclusive("\n")
    }
}
