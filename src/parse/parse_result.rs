pub type ParseResult<I, T> = Result<(I, T), I>;

pub trait Consume: Sized {
    type Quantity;

    /// Consumes the given quantity of items from the input.
    ///
    /// The quantity can be either a number of items or a number of bytes.
    fn consume(self, quantity: Self::Quantity) -> Self;
}

pub fn parsed<C: Consume, T>(consumed: C, quantity: C::Quantity, parsed: T) -> ParseResult<C, T> {
    Ok((consumed, parsed))
}

pub fn failed<C: Consume, T>(consumed: C) -> ParseResult<C, T> {
    Err(consumed)
}
