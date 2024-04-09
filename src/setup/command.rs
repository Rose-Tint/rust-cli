use std::option as std;
use super::help::*;
use super::option::*;


#[derive(Debug, Clone, Copy, Hash)]
pub enum ArgsConstraint {
    Exactly(usize),
    AtLeast(usize),
    AtMost(usize),
}

pub use self::ArgsConstraint::*;

impl Default for ArgsConstraint {
    fn default() -> Self {
        AtLeast(0)
    }
}

/// Subsequent arguments/commands
pub (crate) enum SubArgs {
    Arguments(ArgsConstraint),
    Subcommands(Vec<Command>),
}

pub (crate) use SubArgs::*;

impl Default for SubArgs {
    fn default() -> Self {
        Arguments(ArgsConstraint::default())
    }
}

#[allow(dead_code)]
pub struct Command {
    pub (crate) name: String,
    pub (crate) help: std::Option<String>,
    pub (crate) options: Vec<Option>,
    pub (crate) args: SubArgs,
}

impl Command {
    pub fn new(name: &str) -> Self {
        return Self {
            name: name.to_string(),
            help: None,
            options: Vec::new(),
            args: SubArgs::Arguments(AtLeast(0)),
        };
    }

    pub fn option(mut self, opt: Option) -> Self {
        self.options.push(opt);
        return self;
    }

    pub fn args_constraint(mut self, args: ArgsConstraint) -> Self {
        self.args = SubArgs::Arguments(args);
        return self;
    }

    /// Overwrites the ability to accept arguments. Will overwrite any prior
    /// `arg_constraint` call, but will be overwritten by any subsequent calls
    /// thereof
    pub fn subcommand(mut self, cmd: Command) -> Self {
        self.args = if let Subcommands(mut subcommands) = self.args {
                subcommands.push(cmd);
                Subcommands(subcommands)
            } else {
                Subcommands(vec![cmd])
            };
        return self;
    }
}

impl HasHelp for Command {
    fn get_help(&self) -> Help {
        todo!()
    }

    fn help(mut self, help: &str) -> Self {
        self.help = Some(help.to_string());
        return self;
    }
}
