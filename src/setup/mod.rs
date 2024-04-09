mod command;
mod help;
mod name;
mod option;
pub use command::*;
#[allow(unused_imports)]
pub use help::*;
pub use name::*;
pub use option::*;


pub struct CmdLineSetup {
    pub commands: Vec<Command>,
    pub args: ArgsConstraint,
    /// Universal options
    pub options: Vec<Option>,
}
