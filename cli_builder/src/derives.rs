use cli_lexer::*;


pub trait CmdLine: Sized {
    type CommandType: Command;
    fn parse(args: Vec<String>) -> err::ParseResult<Self>;
}

pub trait Command: Default {
    fn parse_command(iter: CommandIter) -> err::ParseResult<Self>;
}

pub trait Flag {
}

pub fn get_args<T: CmdLine>() -> err::ParseResult<T> {
    <T as CmdLine>::parse(std::env::args().collect())
}

pub mod helpers {
    use std::str::FromStr;
    use cli_lexer::err::*;

    use super::*;

    macro_rules! one_arg {
        (flag $flag:literal => |$arg:ident| $expr:expr) => {
            |args: FlagArgs| args.first().map_or(
                Err(TooManyFlagArgs(flag, args)),
                |$arg| Ok($expr)
            )
        };
    }

    pub fn one_arg<'a, T, F>(flag: &'a str, f: F) -> impl Fn(FlagArgs) -> err::ParseResult<T> + 'a
        where F:Fn(String) -> err::ParseResult<T> + 'a
    {
        return move |args: FlagArgs| match args.split_first() {
            None => Err(MissingFlag(flag.to_string())),
            Some((arg, &[])) => f(arg.clone()),
            Some((arg, _)) => Err(TooManyFlagArgs(flag.to_string(), args)),
        };
    }
}

impl Command for () {
    fn parse_command(_iter: CommandIter) -> err::ParseResult<Self> {
        Ok(())
    }
}

/// for where there are no commands, but accepts only arguments
impl Command for Vec<String> {
    fn parse_command(mut iter: CommandIter) -> err::ParseResult<Self> {
        Ok(iter.collect())
    }
}
