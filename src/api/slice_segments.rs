use core::slice;
use std::iter::FusedIterator;

#[derive(Debug, Clone)]
pub struct SliceSegments<'a> {
    iter: slice::Iter<'a, &'a str>,
}

impl<'a> SliceSegments<'a> {
    pub fn new(iter: slice::Iter<'a, &'a str>) -> Self {
        Self { iter }
    }
}

impl<'a> Iterator for SliceSegments<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|s| *s)
    }
}

impl<'a> From<&'a [&'a str]> for SliceSegments<'a> {
    fn from(slice: &'a [&'a str]) -> Self {
        Self::new(slice.iter())
    }
}

// TODO: implement the rest of the traits for slice iter.
impl<'a> FusedIterator for SliceSegments<'a> {}
