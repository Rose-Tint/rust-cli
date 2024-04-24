#![macro_use]

use crate::reader::*;
use crate::name::*;


pub struct Flag<T> {
    pub (crate) names: Names,
    pub (crate) descr: Option<String>,
    pub (crate) metavar: Option<String>,
    pub (crate) reader: Option<Reader<T>>,
}

impl<T> Flag<T> {
    pub (crate) fn new(name: impl Into<FlagName>) -> Self {
        Flag {
            names: vec![name.into()],
            descr: None,
            metavar: None,
            reader: None,
        }
    }

    // ##### BUILDER FUNCTIONS ######

    pub fn long(mut self, name: &str) -> Self {
        self.names.push(FlagName::Long(name.to_string()));
        return self;
    }

    pub fn short(mut self, name: char) -> Self {
        self.names.push(FlagName::Short(name));
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

#[allow(private_bounds)]
pub fn flag<T: 'static>(name: impl Into<FlagName>, value: T) -> Flag<T> {
    Flag {
        names: vec![name.into()],
        descr: None,
        metavar: None,
        reader: Some(Reader::no_arg(value)),
    }
}

#[allow(private_bounds)]
pub fn option<T: 'static>(name: impl Into<FlagName>) -> Flag<T> {
    Flag {
        names: vec![name.into()],
        descr: None,
        metavar: None,
        reader: None,
    }
}

