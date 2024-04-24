#![macro_use]

use crate::reader::*;
use crate::name::*;
use crate::help::*;


pub struct CmdOption<T> {
    pub (crate) names: Names,
    pub (crate) descr: Option<String>,
    pub (crate) metavar: Option<String>,
    pub (crate) reader: Option<Reader<T>>,
}

impl<T> CmdOption<T> {
    pub (crate) fn new(name: impl Into<Name>) -> Self {
        CmdOption {
            names: vec![name.into()],
            descr: None,
            metavar: None,
            reader: None,
        }
    }

    // ##### BUILDER FUNCTIONS ######

    pub fn long(mut self, name: &str) -> Self {
        self.names.push(Name::Long(name.to_string()));
        return self;
    }

    pub fn short(mut self, name: char) -> Self {
        self.names.push(Name::Short(name));
        return self;
    }

    pub fn metavar(mut self, metavar: &str) -> Self {
        self.metavar = Some(metavar.to_string());
        return self;
    }

    pub fn reader(mut self, rdr: impl Into<Reader<T>>) -> Self {
        self.reader = Some(rdr.into());
        return self;
    }

    pub fn value(mut self, value: T) -> Self
        where T: 'static
    {
        self.reader = Some(Reader::no_arg(value));
        return self;
    }
}

impl<O> HasHelp for CmdOption<O> {
    fn get_help(&self) -> Help {
        Help {
            names: self.names.clone(),
            metavar: self.metavar.clone(),
            descr: descr_helper(self.descr.clone()),
        }
    }

    fn help(mut self, help: &str) -> Self {
        self.descr = Some(help.to_string());
        return self;
    }
}

#[allow(private_bounds)]
pub fn flag<T: 'static>(name: impl Into<Name>, value: T) -> CmdOption<T> {
    CmdOption {
        names: vec![name.into()],
        descr: None,
        metavar: None,
        reader: Some(Reader::no_arg(value)),
    }
}

#[allow(private_bounds)]
pub fn option<T: 'static>(name: impl Into<Name>) -> CmdOption<T> {
    CmdOption {
        names: vec![name.into()],
        descr: None,
        metavar: None,
        reader: None,
    }
}

