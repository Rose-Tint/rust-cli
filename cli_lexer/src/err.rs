use crate::{common::*, FlagArgs};

#[derive(Debug)]
pub enum CmdLineErr {
    UnrecognizedCommand(Raw),
    TooManyFlagArgs(Raw, Vec<Raw>),
    MissingFlagArg(Raw),
    UnrecognizedFlag(Raw),
    MissingFlag(Raw),
    FlagSetTwice(Raw),
    ExpectedFlag(Raw),
}

pub use CmdLineErr::*;

pub type ParseResult<T> = std::result::Result<T, CmdLineErr>;

pub struct Usage();

pub enum Context {
    OnFlag {
        flag: Raw,
        args: FlagArgs,
        usage: Usage,
    },
    OnCommand {}
}

pub trait ErrFormatter {
    fn format(err: CmdLineErr) -> String;
}
