#[cfg(test)]
mod tests {
    use cli::*;
    use cli::err::*;
    #[allow(unused)]
    use cli::derives::{helpers::*, CmdLine};

    macro_rules! command_line {
        ($($args:literal)*) => {
            <TestConfig as CmdLine>
            ::parse(string_vec![ $( $args, )* ])
        };
    }

    macro_rules! string_vec {
        ($( $s:literal, )*) => {
            string_vec![ $( $s ),* ]
        };
        ($( $s:literal ),*) => {
            vec![ $( $s.to_string() ),* ]
        };
    }

    #[derive(Debug, CmdLine)]
    struct TestConfig {
        /// does not accept any arguments
        #[flag(long="none", value=true, default=false)]
        no_args: bool,

        /// requires exactly one arg
        #[flag(long="one", parser=one_arg("one", |arg| Ok(arg)))]
        one_arg: String,

        /// can take any number of args
        #[flag(long="many", parser=|args| Ok(args))]
        many_args: Vec<String>
    }

    #[test]
    fn flags_eq() {
        let cfg = command_line![
            "--none"
            "--one" "arg1"
            "--many" "arg1" "arg2" "arg3"
            ]
            .expect("Command line macro failed");
        assert_eq!(cfg.no_args, true, "no arguments");
        assert_eq!(cfg.one_arg, "arg1", "one argument");
        assert_eq!(cfg.many_args, vec![
            "arg1".to_string(),
            "arg2".to_string(),
            "arg3".to_string()]
            ,
            "many arguments"
        );
    }

    #[test]
    fn none_gets_one() {
        let cmd = command_line![ "--none" "arg" ];
        if cmd.is_err() {
            println!("Result: {cmd:?}");
        }
        assert!(cmd.is_err_and(|err| is_too_many_flag_args("--none", err)))
    }

    #[test]
    fn one_gets_none() {
        let cmd = command_line![ "--one" ];
        if cmd.is_err() {
            println!("Result: {cmd:?}");
        }
        assert!(cmd.is_err_and(|err| is_missing_flag_arg("--one", err)))
    }

    #[test]
    fn one_gets_multiple() {
        let cmd = command_line![ "--one" "arg1" "arg2" ];
        if cmd.is_err() {
            println!("Result: {cmd:?}");
        }
        assert!(cmd.is_err_and(|err| is_too_many_flag_args("one", err)))
    }

    pub fn is_too_many_flag_args(flag: &str, err: CmdLineErr) -> bool {
        match err {
            TooManyFlagArgs(err_flag, _) => flag == err_flag,
            _ => false,
        }
    }

    pub fn is_missing_flag_arg(flag: &str, err: CmdLineErr) -> bool {
        match err {
            MissingFlagArg(err_flag) => flag == err_flag,
            _ => false,
        }
    }
}
