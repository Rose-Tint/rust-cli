use crate::common::*;


pub struct CommandIter {
    args: Box<[Raw]>,
    index: usize,
    size: usize,
}

impl CommandIter {
    pub fn new(args: Box<[Raw]>) -> Self {
        Self { index: 0, size: args.len(), args }
    }
}

impl Iterator for CommandIter {
    type Item = Raw;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.size {
            None
        } else {
            Some((&self.args[self.index]).clone())
        }
    }
}
