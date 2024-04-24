#![allow(dead_code)]

pub use crate::err::CmdLineErr;
use crate::common::*;

mod commanditer;
pub use commanditer::*;
mod flagiter;
pub use flagiter::*;


pub struct CommandLine {
    pub command_iter: CommandIter,
    pub flag_iter: FlagIter,
}

impl CommandLine {
    pub fn from_iter<I, F, T>(iter: I, is_valid_flag: F) -> Self
    where
        I: Iterator<Item=T>,
        T: Into<Raw>,
        F: Fn(&Raw) -> bool + 'static,
    {
        let vec: Vec<Raw> = iter.map(Into::into).collect();
        let (pre_flags, post_flags) = split_when(&vec, &is_valid_flag);
        let command_iter: CommandIter = CommandIter::new(pre_flags);
        let flag_iter: FlagIter = FlagIter::new(post_flags, is_valid_flag);
        CommandLine { command_iter, flag_iter }
    }
}

/// custom implementation because rust's is not applicable. the first match is
/// included in the second value of the tuple.
fn split_when(vec: &Vec<Raw>, predicate: impl Fn(&Raw) -> bool) -> (Box<[Raw]>, Box<[Raw]>) {
    let pos = vec.iter()
        .position(|x| predicate(&x))
        .unwrap_or(vec.len());
    let (a, b) = vec.split_at(pos);
    return (a.into(), b.into());
}
