use std::option as std;
use crate::name::*;
use crate::reader;

use crate::help::*;
use crate::option::*;


pub type ArgsReader<O> = std::Option<reader::Reader<O, Vec<String>>>;

/// @type-param C - the type representing the commands
/// @type-param O - The type used to store the chosen options.
#[allow(dead_code)]
pub struct Command<C, O=C> {
    pub (crate) names: Names,
    pub (crate) value: C,
    pub (crate) args_reader: ArgsReader<C>,
    pub (crate) help: std::Option<String>,
    pub (crate) options: Vec<Option<O>>,
}

impl<C, O> Command<C, O> {
    pub fn new(name: &str, value: C) -> Self {
        return Self {
            names: vec![Name::Unprefixed(name.to_string())],
            value,
            args_reader: None,
            help: None,
            options: Vec::new(),
        };
    }

    pub fn alias(mut self, name: &str) -> Self {
        self.names.push(Name::Unprefixed(name.to_string()));
        return self;
    }

    pub fn option(mut self, opt: Option<O>) -> Self {
        self.options.push(opt);
        return self;
    }

    pub fn argument_reader(mut self, args_reader: ArgsReader<C>) -> Self {
        self.args_reader = args_reader;
        return self;
    }

}

impl<O> HasHelp for Command<O> {
    fn get_help(&self) -> Help {
        todo!()
    }

    fn help(mut self, help: &str) -> Self {
        self.help = Some(help.to_string());
        return self;
    }
}
