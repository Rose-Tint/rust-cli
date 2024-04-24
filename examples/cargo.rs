#![allow(dead_code)]
#![allow(unused)]

extern crate cli;

use cli::builder::command::*;
use cli::builder::option::*;
use cli::builder::reader::{*, result::Result};


fn main() {
}

enum Commands {
    Build(Vec<String>),
    Install(Vec<String>),
    Uninstall(Vec<String>),
    Run(Vec<String>),
}

use Commands::*;

enum Options {
    Silent,
    Verbose,
    BuildFolder(String),
    Threaded,
    Profile,
    Jobs(u8),
}

fn read_jobs(s: String) -> Result<Options> {
    return match s.parse::<u8>() {
        Err(e) => Err(Custom(e.to_string())),
        Ok(x) => Ok(Jobs(x)),
    }
}

fn read_build_folder(oarg: std::option::Option<String>) -> Result<Options> {
    return Ok(match oarg {
        None => BuildFolder("/target/".into()),
        Some(arg) => BuildFolder(arg)
    })
}

use Options::*;

fn setup_config() -> cli::builder::Config<Commands, Options> {
    cli::builder::setup()
        .command(Command::new("build", Build(vec![])))
        .command(Command::new("install", Install(vec![])))
        .command(Command::new("uninstall", Uninstall(vec![])))
        .command(Command::new("run", Run(vec![])))
        .option(flag("profile", Profile)
            .short('P'))
        .option(flag("threaded", Threaded))
        .option(option("jobs")
            .short('j')
            .reader(Reader::optional(Jobs(0), read_jobs)))
        .option(option("build").reader(Reader::new(read_build_folder)))
        .option(flag("verbose", Verbose).short('v'))
        .option(flag("silent", Silent).short('s'))
}
