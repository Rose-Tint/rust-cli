use std::option as std;

use crate::name::*;
// use crate::reader;
// use crate::option::*;


/// @type-param C - the type representing the commands
pub struct Command<C> {
    pub (crate) names: String,
    pub (crate) value: C,
    // pub (crate) args_reader: Reader<C>,
    pub (crate) help: std::Option<String>,
    // pub (crate) options: Vec<Option<O>>,
}

impl<C> Command<C> {
    pub fn new(name: &str, value: C) -> Self {
        return Self {
            names: name.to_string(),
            value,
            // args_reader: None,
            help: None,
            // options: Vec::new(),
        };
    }

    // pub fn option(mut self, opt: Option<O>) -> Self {
    //     self.options.push(opt);
    //     return self;
    // }

    // pub fn argument_reader(mut self, args_reader: Reader<C>) -> Self {
    //     self.args_reader = args_reader;
    //     return self;
    // }

}
