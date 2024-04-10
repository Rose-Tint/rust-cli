use crate::reader::*;
use crate::name::*;
use crate::help::*;
use std::option as std;


pub type OptReader<O> = Reader<O, std::Option<String>>;

#[allow(dead_code)]
pub struct Option<O> {
    pub (crate) names: Names,
    pub (crate) help: std::Option<String>,
    pub (crate) metavar: std::Option<String>,
    pub (crate) reader: OptReader<O>,
}

impl<O> Option<O> {
    #[allow(private_bounds)]
    pub fn new<N: Into<Name>>(name: N, rdr: OptReader<O>) -> Self {
        Option {
            names: vec![name.into()],
            help: None,
            metavar: None,
            reader: rdr
        }
    }

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
}

#[allow(private_bounds)]
pub fn flag<N, O, F>(name: N, f: F) -> Option<O>
where
    F: FnOnce(O, bool) -> RdrResult<O> + 'static,
    N: Into<Name>
{
    Option {
        names: vec![name.into()],
        help: None,
        metavar: None,
        reader: Reader::bool_reader(f),
    }
}

#[allow(private_bounds)]
pub fn switch<N, O, F>(name: N, f: F) -> Option<O>
where
    F: FnOnce(O) -> RdrResult<O> + 'static,
    N: Into<Name>
{
    Option {
        names: vec![name.into()],
        help: None,
        metavar: None,
        reader: Reader::bool_reader(|opts, _| f(opts)),
    }
}

impl<O> HasHelp for Option<O> {
    fn get_help(&self) -> Help {
        return Help {
            names: self.names.clone(),
            metavar: self.metavar.clone(),
            descr: descr_helper(self.help.clone()),
        };
    }

    fn help(mut self, help: &str) -> Self {
        self.help = Some(help.to_string());
        return self;
    }
}
