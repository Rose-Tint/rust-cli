use std::path::Path;

extern crate cli;
// use cli::setup::*;

fn main() {
}

enum Command {
    Help(Option<Command>),
    Build(BuildType),
    Install(Vec<String>),
    Update,
    Run(Option<String>),
}

enum BuildType {
    Library,
    Tests,
    Executable,
}

struct CmdLine {
    verbose: bool,
    output_folder: Path,
    jobs: usize,
}

fn get_cmdline() {
    use cli::setup::*;
    let setup = CmdLineSetup {
        commands: vec![
        ],
        args: ArgsConstraint::AtLeast(0),
        options: vec![
            flag()
                .long("verbose")
                .short('v')
                .help("Enable verbose output")
        ]
    };
}
