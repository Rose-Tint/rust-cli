pub mod command;
pub mod help;
pub mod name;
pub mod option;
pub mod reader;

use crate::command::*;
use crate::option::*;

pub struct Setup<C, O=C> {
    pub commands: Vec<Command<C, O>>,
    pub options: Vec<Option<O>>,
}

pub fn setup<C, O>() -> Setup<C, O> {
    Setup { commands: vec![], options: vec![] }
}

impl<C, O> Setup<C, O> {
    pub fn command(mut self, cmd: Command<C, O>) -> Self {
        self.commands.push(cmd);
        return self;
    }

    pub fn option(mut self, opt: Option<O>) -> Self {
        self.options.push(opt);
        return self;
    }
}
