use crate::{
    ast::block::Block,
    parse::phase_1::block::{OpenBlock, OpenLeaf},
};
use std::ops::ControlFlow;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Document<'a> {
    _phantom: std::marker::PhantomData<&'a ()>,
}

impl<'a> Document<'a> {
    pub fn new() -> Self {
        Document {
            _phantom: std::marker::PhantomData,
        }
    }

    pub fn open() -> OpenDocument<'a> {
        OpenDocument::new()
    }
}

impl<'a> From<OpenDocument<'a>> for Document<'a> {
    fn from(_: OpenDocument<'a>) -> Self {
        Document::new()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OpenDocument<'a> {
    closed: Vec<Block<'a>>,
    current: Option<OpenBlock<'a>>,
}

impl<'a> OpenDocument<'a> {
    pub fn new() -> Self {
        Self {
            closed: Vec::new(),
            current: None,
        }
    }

    pub fn visit_current_branch<B, F: FnMut(&mut OpenBlock<'a>) -> ControlFlow<B>>(
        &mut self,
        mut visitor: F,
    ) -> ControlFlow<B> {
        let mut current = self.current.as_mut();
        while let Some(block) = current {
            visitor(block)?;
            current = match block {
                OpenBlock::Container(container) => container.current(),
                OpenBlock::Leaf(_) => None,
            };
        }
        ControlFlow::Continue(())
    }

    pub fn current_leaf(&mut self) -> Option<&mut OpenLeaf<'a>> {
        let mut current = self.current.as_mut();
        while let Some(block) = current {
            current = match block {
                OpenBlock::Container(container) => container.current(),
                OpenBlock::Leaf(leaf) => return Some(leaf),
            };
        }
        None
    }
}
