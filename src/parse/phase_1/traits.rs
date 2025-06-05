use crate::ast::block::Block;

pub trait IOpenBlock<'a> {
    fn stage(&mut self, line: &'a str) -> Result<&'a str, ()>;
    fn commit(&mut self);
    fn close(self) -> Block<'a>;
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Staging<'a> {
    line: Option<&'a str>,
}

impl<'a> Staging<'a> {
    pub fn set(&mut self, line: &'a str) {
        if self.line.is_some() {
            panic!("staging already set");
        }
        self.line = Some(line);
    }

    pub fn commit<F: FnOnce(&'a str) -> ()>(&mut self, sink: F) {
        sink(self.line.take().expect("cannot commit empty staging"))
    }

    pub fn has_line_staged(&self) -> bool {
        self.line.is_some()
    }

    pub fn reset(&mut self) {
        if self.line.is_none() {
            panic!("cannot reset empty staging");
        }
        self.line = None;
    }
}
