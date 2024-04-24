#![allow(dead_code)]
#![allow(unused)]

extern crate cli;

use cli::*;
use cli::derives::helpers::*;


fn main() { }

#[derive(CmdLine)]
// #[version = "0.1.0.0"]
struct Config {
    #[command]
    command: (), // Commands,

    /// Use verbose output
    #[flag(long="verbose", short='v', default=Normal, value=Verbose)]
    // /// Do not print cargo log messages
    // #[flag(long="quiet", short='q')] // aliases not yet implemented
    // #[all(default=Normal)]
    verbosity: Verbosity,

    // /// Optimization level
    // #[flag(short='O', default=0, value=2, parser=|s: Raw|Ok(s.parse::<u8>().clamp(0,3)))]
    // optimization: u8,

    /// The folder to put build files in
    #[flag(long="build-folder", default="/build/", parser=one_arg("build-folder", |s| Ok(s)))]
    build_folder: String,

    /// Run without accessing the network
    #[flag(long="offline", default=false, value=true)]
    offline: bool,
}

#[derive(Command, Default)]
enum Commands {
    /// Compile a local package and all of its dependencies
    Build,
    /// Install a Rust binary. Default location is $HOME/.cargo/bin
    Install,
    /// Remove a Rust binary
    Uninstall,
    /// Run a binary or example of the local package
    #[default] Run,
}

#[derive(Flag)]
enum Verbosity {
    #[value("0")] Silent,
    #[value("1")] Quiet,
    #[value("2")] Normal,
    #[value("3")] Verbose,
    #[value("4")] Debug,
    #[value("5")] Trace,
}
use Verbosity::*;
