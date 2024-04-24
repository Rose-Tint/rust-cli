use crate::command::*;
use crate::option::*;


pub struct Config<C, O> {
    pub commands: Vec<Command<C>>,
    pub options: Vec<Flag<O>>,
}

pub fn setup<C, O>() -> Config<C, O> {
    Config { commands: vec![], options: vec![] }
}

impl<C, O> Config<C, O> {
    pub fn command(mut self, cmd: Command<C>) -> Self {
        self.commands.push(cmd);
        return self;
    }

    pub fn option(mut self, opt: Flag<O>) -> Self {
        self.options.push(opt);
        return self;
    }
}
