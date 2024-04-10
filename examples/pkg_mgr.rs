use cli::command::*;
use cli::option::*;

extern crate cli;

fn main() {
}

enum Commands {
    Build(Vec<String>),
    Install(Vec<String>),
    Uninstall(Vec<String>),
    Run(Vec<String>),
}

use Commands::*;

enum Verbosity {
    Verbose,
    Normal,
    Silent,
}

struct Options {
    verbosity: Verbosity,
    output_folder: String,
    threaded: bool,
    profile: bool,
    jobs: u8,
}

fn setup_config() -> cli::Setup<Commands, Options> {
    cli::setup()
        .command(
            Command::new("build", Build(vec![]))
                .option(switch(
                    "profile",
                    |opts| {
                        opts.profile = true;
                        return opts
                    }))
        )

    // return Setup {
    //     commands: vec![
    //     ],
    //     args: ArgsConstraint::AtLeast(0),
    //     options: vec![
    //         flag()
    //             .long("verbose")
    //             .short('v')
    //             .help("Enable verbose output")
    //     ]
    // };
}
