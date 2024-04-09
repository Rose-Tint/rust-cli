use crate::validator::*;
use super::name::*;
use super::help::*;
use std::option as std;


#[allow(dead_code)]
pub struct Option {
    pub (crate) names: Names,
    pub (crate) help: std::Option<String>,
    pub (crate) metavar: std::Option<String>,
    pub (crate) validator: Validator,
}

pub fn option() -> Option {
    return Option {
        names: vec![],
        help: None,
        metavar: None,
        validator: Validator::optional(|_|true)
    }
}

pub fn flag() -> Option {
    return Option {
        names: vec![],
        help: None,
        metavar: None,
        validator: Validator::regex(r"(true|false)?"),
    }
}

impl Option {
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

impl HasHelp for Option {
    fn get_help(&self) -> Help {
        return Help {
            names: self.names.clone(),
            metavar: self.metavar.clone(),
            descr: self.help.clone().unwrap_or(default_help()),
        };
    }

    fn help(mut self, help: &str) -> Self {
        self.help = Some(help.to_string());
        return self;
    }
}
